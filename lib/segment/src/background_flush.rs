//! Background flush (prototype).

use std::thread;

pub fn spawn_flush() {
    let _ = thread::spawn(|| {});
}
