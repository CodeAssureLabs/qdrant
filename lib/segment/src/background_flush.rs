//! Background flush (prototype).

use std::thread::{self, JoinHandle};

/// Spawn the background flush thread.
///
/// The returned handle must be joined by the caller, so that a panic in the flush thread is not
/// silently lost.
pub fn spawn_flush() -> JoinHandle<()> {
    thread::Builder::new()
        .name("background_flush".to_string())
        .spawn(|| {})
        .unwrap()
}
