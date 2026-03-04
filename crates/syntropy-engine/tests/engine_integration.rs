use kameo::Actor;
use syntropy_engine::actor::{coder::CoderAgent, router::RouterAgent};
use std::time::Duration;
use tracing::info;

#[tokio::test]
async fn test_actor_lifecycle_and_messaging() -> Result<(), Box<dyn std::error::Error>> {
    // We don't initialize full tracing-subscriber in tests lightly but we can
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_test_writer()
        .try_init();

    info!("Starting test_actor_lifecycle_and_messaging");

    // Spawn CoderAgent
    let coder_actor = CoderAgent {
        name: "test_coder".to_string(),
        active_task: None,
    };
    let coder_ref = kameo::spawn(coder_actor);

    // Spawn RouterAgent
    let router_actor = RouterAgent {
        name: "test_router".to_string(),
        subordinates: vec![],
    };
    let router_ref = kameo::spawn(router_actor);

    // Give actors a moment to initialize
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send a message to Coder (normally done by router, but we test direct delivery first)
    let msg = syntropy_engine::actor::TaskRequest {
        task_id: "test-task-1".to_string(),
        instruction: "Write a hello world program".to_string(),
    };
    
    // Asynchronously call the coder
    let reply = coder_ref.ask(msg).await?;
    info!("Coder replied: {:?}", reply);
    
    assert!(reply.is_ok());
    assert!(reply.unwrap().contains("test-task-1"));

    // Shutdown gracefully
    coder_ref.kill();
    router_ref.kill();

    Ok(())
}
