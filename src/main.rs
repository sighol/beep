use rand::prelude::*;
use rodio::Source;
use std::time;

fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    babubabu(&sink);
    add_sound(&sink, 440.0, 100);

    sink.set_volume(0.5);
    sink.sleep_until_end();
}

fn babubabu(sink: &rodio::Sink) {
    let start = 440.0 * 2.0;
    let end = 440.0;
    let num = 5;
    for _ in 0..1 {
        for i in 0..(num + 1) {
            let freq = ((num - i) as f64 / num as f64) * (start - end) + end;
            let freq = freq as f32;
            add_sound(&sink, freq, 30);
        }
    }
}

fn random(sink: &rodio::Sink) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let freq: f64 = rng.gen();
        let freq = 200.0 + (freq * 1000.0);
        add_sound(&sink, freq as f32, 120);
    }
}

fn add_sound(sink: &rodio::Sink, freq: f32, millis: u64) {
    let source = rodio::source::SineWave::new(freq);
    let source = source.take_duration(time::Duration::from_millis(millis));
    sink.append(source);
}
