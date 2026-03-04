use anyhow::Result;
use wasmtime::*;

pub struct WasmSandbox {
    engine: Engine,
}

impl WasmSandbox {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        // Enable fuel consumption to prevent infinite loops
        config.consume_fuel(true);
        // Optimize for speed
        config.cranelift_opt_level(OptLevel::Speed);

        let engine = Engine::new(&config)?;
        Ok(Self { engine })
    }

    /// Executes a WASM blob, limiting execution via fuel
    pub fn execute_wasm(&self, wasm_bytes: &[u8], fuel_limit: u64) -> Result<String> {
        let mut store = Store::new(&self.engine, ());
        store.set_fuel(fuel_limit)?;

        let module = Module::new(&self.engine, wasm_bytes)?;

        // No WASI overhead for simple execution MVP

        let instance = Instance::new(&mut store, &module, &[])?;
        let func = instance.get_typed_func::<(), i32>(&mut store, "run")?;

        let result = func.call(&mut store, ())?;

        Ok(format!("Execution result state: {}", result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_execution() -> Result<()> {
        let sandbox = WasmSandbox::new()?;

        // Simple WAT that exports a "run" function returning 42
        let wat = r#"
            (module
                (func (export "run") (result i32)
                    i32.const 42
                )
            )
        "#;

        let wasm_bytes = wat::parse_str(wat)?;

        let result = sandbox.execute_wasm(&wasm_bytes, 1000)?;
        assert_eq!(result, "Execution result state: 42");

        Ok(())
    }
}
