use steno::{ActionRegistry, SagaDag, SagaId, SagaName, SagaType, SecClient};
use std::sync::Arc;
#[derive(Debug)]
pub struct SyntropyContext;
impl SagaType for SyntropyContext {}
fn check(client: &SecClient) {
    let dag = Arc::new(SagaDag::new(
        vec![].into_iter(),
        SagaName::new("DeveloperWorkflow"),
    ).unwrap());
    let registry: Arc<ActionRegistry<SyntropyContext>> = Arc::new(ActionRegistry::new());
    let saga_id = SagaId::new();
}
