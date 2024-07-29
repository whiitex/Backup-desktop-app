use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub fn play_sound() {
    // Use rodio to play a sound
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let file = BufReader::new(File::open("assets/beep.wav").unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);

    // Sleep for the duration of the sound to ensure it plays completely
    sink.sleep_until_end();
}