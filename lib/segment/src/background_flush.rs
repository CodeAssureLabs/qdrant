//! Background flush (prototype).

/// Schedule the flush on the rayon pool rather than a raw OS thread.
pub fn spawn_flush() {
    rayon::spawn(|| {});
}
