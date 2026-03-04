use syntropy_engine::actor::coder::CoderAgent;
use syntropy_engine::saga::SagaOrchestrator;
use syntropy_engine::actor::TaskRequest;
use steno::InMemorySecStore;
use kameo::spawn;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use slog::{o, Logger, Discard};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Starting Actor + Saga Example");

    // 1. Start Steno SecClient for Sagas
    let log = Logger::root(Discard, o!());

    let sec_store = Arc::new(InMemorySecStore::new());
    let sec_client = steno::sec(log, sec_store);

    let orchestrator = SagaOrchestrator::new(sec_client);

    // 2. Start Kameo CoderAgent Actor
    info!("Spawning CoderAgent actor");
    let coder_actor = spawn(CoderAgent::default());

    // 3. Trigger a Saga Orchestration directly
    let task_id = "TASK-001";
    info!("Initiating saga for task {}", task_id);
    let saga_id = orchestrator.start_research_and_code_saga(task_id).await?;
    info!("Started Saga ID: {:?}", saga_id);

    // Let the saga run for a bit
    sleep(Duration::from_millis(500)).await;

    // 4. Send a task to the actor (simulating a step in the workflow)
    info!("Sending task request to CoderAgent");
    let result = coder_actor.ask(TaskRequest {
        task_id: task_id.to_string(),
        instruction: "Implement feature X".to_string(),
    }).await?;
    
    info!("CoderAgent response: {:?}", result);

    // Let things settle
    sleep(Duration::from_millis(100)).await;

    info!("Actor + Saga Example Finished");
    Ok(())
}
