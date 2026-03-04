use kameo::Actor;
use syntropy_engine::actor::{coder::CoderAgent, router::RouterAgent};
use syntropy_engine::actor::TaskRequest;
use std::time::Duration;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_test_writer() // To make output visible
        .init();

    info!("Starting multi-agent demo!");

    // Spawn actors
    let coder_actor = CoderAgent {
        name: "test_coder".to_string(),
        active_task: None,
    };
    let coder_ref = kameo::spawn(coder_actor);

    // Sleep let init
    tokio::time::sleep(Duration::from_millis(50)).await;

    let msg = TaskRequest {
        task_id: "example-task-1337".to_string(),
        instruction: "Compile this example successfully".to_string(),
    };
    
    // In demo, we just directly prompt the active CoderAgent
    info!("Assigning task to coder...");
    let reply = coder_ref.ask(msg).await?;
    info!("Coder complete. Response: {:?}", reply);

    tokio::time::sleep(Duration::from_millis(50)).await;

    // Shutdown gracefully
    coder_ref.kill();

    Ok(())
}
