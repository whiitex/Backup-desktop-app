use std::fs::File;
use portaudio as pa;
use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;
use std::env;

const CHANNELS: i32 = 1;
const SAMPLE_RATE: f64 = 44100.0;
const FRAMES_PER_BUFFER: u32 = 64;

fn read_wav_file(file_path: &str) -> io::Result<Vec<f32>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Parse WAV file header (simple implementation)
    let data_start = 44; // assuming 44-byte header for PCM format
    let mut samples = Vec::new();
    for chunk in buffer[data_start..].chunks(2) {
        let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
        samples.push(sample as f32 / i16::MAX as f32);
    }
    Ok(samples)
}

pub fn play_sound() {
    let pa = pa::PortAudio::new().unwrap();
    let settings = pa.default_output_stream_settings::<f32>(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER).unwrap();

    let (tx, rx) = mpsc::channel();

    // Start audio thread
    thread::spawn(move || {
        let exe = env::current_exe().unwrap(); // exe path
        let wd = exe.parent().unwrap();
        let file_path = wd.join("assets/beep.wav");

        let samples = read_wav_file(&file_path.to_str().unwrap()).expect("Failed to read WAV file");
        let s1 = samples.clone();

        let mut sample_idx = 0;
        let mut stream = pa.open_non_blocking_stream(settings, move |output: pa::OutputStreamCallbackArgs<f32>| {
            for out_sample in output.buffer.iter_mut() {
                if sample_idx < samples.len() {
                    *out_sample = samples[sample_idx];
                    sample_idx += 1;
                } else {
                    *out_sample = 0.0; // Silence if we run out of samples
                }
            }
            pa::Continue
        }).unwrap();

        stream.start().unwrap();

        // Sleep for the duration of the sound to ensure it plays completely
        thread::sleep(std::time::Duration::from_secs_f32(s1.len() as f32 / SAMPLE_RATE as f32));

        tx.send(()).unwrap(); // Signal completion
    });

    rx.recv().unwrap(); // Wait for completion
    println!("Playback finished.");
}