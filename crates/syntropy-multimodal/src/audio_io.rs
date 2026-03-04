use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use tracing::{info, warn};

pub struct AudioCapture {
    pub is_recording: Arc<Mutex<bool>>,
    // buffer to store f32 samples
    pub buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioCapture {
    pub fn new() -> Self {
        Self {
            is_recording: Arc::new(Mutex::new(false)),
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Starts capturing audio using the default input device
    pub fn start(&self) -> Result<()> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;

        let config = device.default_input_config()?;

        info!("Starting audio capture with config: {:?}", config);

        let mut _is_rec = self.is_recording.lock().unwrap();
        *_is_rec = true;

        // In a real implementation this stream would be kept alive in a background thread.
        // For the MVP scaffold, we log that it started.

        Ok(())
    }

    pub fn stop(&self) {
        let mut _is_rec = self.is_recording.lock().unwrap();
        *_is_rec = false;
        info!("Stopped audio capture.");
    }

    pub fn get_samples(&self) -> Vec<f32> {
        let buf = self.buffer.lock().unwrap();
        buf.clone()
    }
}
