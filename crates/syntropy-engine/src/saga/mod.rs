use steno::{ActionRegistry, SagaType, SecClient, SagaDag, Action, ActionContext};
use std::sync::Arc;
use tracing::info;

#[derive(Debug)]
pub struct SyntropyContext;

impl steno::SagaType for SyntropyContext {
    type ExecContextType = ();
}

#[derive(Debug)]
pub struct ResearchAction;
impl Action<SyntropyContext> for ResearchAction {
    fn name(&self) -> steno::ActionName { steno::ActionName::new("ResearchRequirements") }
    fn do_it(
        &self,
        _action_context: ActionContext<SyntropyContext>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<serde_json::Value>, steno::ActionError>> + Send>> {
        Box::pin(async { Ok(Arc::new(serde_json::json!({}))) })
    }
    fn undo_it(
        &self,
        _action_context: ActionContext<SyntropyContext>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), anyhow::Error>> + Send>> {
        Box::pin(async { Ok(()) })
    }
}

#[derive(Debug)]
pub struct CodeAction;
impl Action<SyntropyContext> for CodeAction {
    fn name(&self) -> steno::ActionName { steno::ActionName::new("ImplementCode") }
    fn do_it(
        &self,
        _action_context: ActionContext<SyntropyContext>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<serde_json::Value>, steno::ActionError>> + Send>> {
        Box::pin(async { Ok(Arc::new(serde_json::json!({}))) })
    }
    fn undo_it(
        &self,
        _action_context: ActionContext<SyntropyContext>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), anyhow::Error>> + Send>> {
        Box::pin(async { Ok(()) })
    }
}

/// Registers all compensable agent actions into the Steno Registry
pub fn register_saga_actions(registry: &mut ActionRegistry<SyntropyContext>) {
    registry.register(Arc::new(ResearchAction));
    registry.register(Arc::new(CodeAction));
}

/// A centralized orchestrator for executing Sagas across different subsystems
pub struct SagaOrchestrator {
    pub client: SecClient,
}

impl SagaOrchestrator {
    pub fn new(client: SecClient) -> Self {
        Self { client }
    }

    /// Example: Initiating a long-running research + coding task
    pub async fn start_research_and_code_saga(&self, task_id: &str) -> Result<steno::SagaId, steno::ActionError> {
        info!("Initiating Research -> Code Saga for Task: {}", task_id);
        
        // Define Saga Workflow
        let research_action: Arc<dyn Action<SyntropyContext>> = Arc::new(ResearchAction);
        let code_action: Arc<dyn Action<SyntropyContext>> = Arc::new(CodeAction);

        let mut builder = steno::DagBuilder::new(steno::SagaName::new("DeveloperWorkflow"));
        builder.append(steno::Node::action(
            "ResearchRequirementsNode",
            "Gather requirements",
            research_action.as_ref()
        ));
        builder.append(steno::Node::action(
            "ImplementCodeNode",
            "Write code logic",
            code_action.as_ref()
        ));
        
        let dag = Arc::new(steno::SagaDag::new(
            builder.build().map_err(|e| steno::ActionError::action_failed(format!("DAG build failed: {:?}", e)))?,
            serde_json::json!({})
        ));

        // Saga ID parsing
        let raw_uuid = uuid::Uuid::new_v4();
        let saga_id = steno::SagaId(raw_uuid);

        // In a real agent workflow, the registry would be passed into the SagaOrchestrator.
        let mut registry = steno::ActionRegistry::new();
        register_saga_actions(&mut registry);
        let registry: Arc<steno::ActionRegistry<SyntropyContext>> = Arc::new(registry);

        let _saga_future = self.client.saga_create(
            saga_id,
            Arc::new(()), // uctx
            dag, // dag
            registry, // registry
        ).await.map_err(|e| steno::ActionError::action_failed(format!("Saga create failed: {:?}", e)))?;

        self.client.saga_start(saga_id).await.map_err(|e| steno::ActionError::action_failed(e.to_string()))?;

        // Await the saga completion if necessary, or just return the ID

        Ok(saga_id)
    }
}
