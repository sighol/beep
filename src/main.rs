use rodio::Source;
use std::io::Cursor;
use std::{thread, time};

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let bytes = include_bytes!("tone.ogg");
    let cursor = Cursor::new(bytes.iter());
    let source = rodio::Decoder::new(cursor).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();

    let dt = time::Duration::from_millis(1471);
    thread::sleep(dt);
}
