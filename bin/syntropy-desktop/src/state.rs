use gpui::EntityId;
use std::sync::{Arc, Mutex};

/// Global Application State
///
/// We use a centralized state model wrapped in Arc<Mutex> for thread-safe access
/// across async Actor tasks and synchronous UI render passes. The `notify_entity`
/// stores the root view ID to trigger efficient zero-cost repaints.
#[derive(Default, Clone)]
pub struct AppState {
    pub status_message: Arc<Mutex<String>>,
    pub is_processing: Arc<Mutex<bool>>,
    // This holds the root EntityId to notify the GPUI runtime of changes
    pub notify_entity: Arc<Mutex<Option<EntityId>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            status_message: Arc::new(Mutex::new("Ready".to_string())),
            is_processing: Arc::new(Mutex::new(false)),
            notify_entity: Arc::new(Mutex::new(None)),
        }
    }
}

impl gpui::Global for AppState {}
