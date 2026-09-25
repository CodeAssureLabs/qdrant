//! Background flush (prototype).

pub fn spawn_flush() {
    std::thread::Builder::new()
        .name("background_flush".to_string())
        .spawn(|| {})
        .unwrap();
}
