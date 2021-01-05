use rodio::Source;
use std::time;
use rand::prelude::*;

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    let mut rng = rand::thread_rng();

    for i in 0..10 {
        let freq: f64 = rng.gen();
        let freq = 200 + ((freq * 1000.0) as u32);
        add_sound(&sink, freq, 120);
    }
    
    sink.set_volume(0.5);
    sink.sleep_until_end();
}

fn add_sound(sink: &rodio::Sink, freq: u32, millis: u64) {
    let source = rodio::source::SineWave::new(freq);
    let source = source.take_duration(time::Duration::from_millis(millis));
    sink.append(source);
}
