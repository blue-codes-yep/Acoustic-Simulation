use cpal::traits::StreamTrait;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Stream, StreamConfig};
use std::sync::Arc; // Import the StreamTrait trait

pub fn generate_wave(sample_rate: f32, frequency: f32, volume: f32) -> Vec<f32> {
    let phase_inc = 2.0 * std::f32::consts::PI * frequency / sample_rate;
    let mut phase: f32 = 0.0;

    let mut wave = Vec::new();
    for _ in 0..(sample_rate as usize) {
        let value = volume * phase.sin();
        wave.push(value);
        phase = (phase + phase_inc) % (2.0 * std::f32::consts::PI);
    }

    wave
}

pub fn play_wave(wave: Vec<f32>) -> Result<Stream, Box<dyn std::error::Error>> {
    println!("Starting play_wave function"); // Add debug print

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let config = StreamConfig {
        channels: 2,
        sample_rate: cpal::SampleRate(44100),
        buffer_size: cpal::BufferSize::Default,
    };

    let wave_arc = Arc::new(wave);
    let wave_clone = wave_arc.clone();

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            println!("Playing wave"); // Add debug print
            for (frame, sample) in data.chunks_mut(2).zip(wave_clone.iter().cycle()) {
                for frame_sample in frame.iter_mut() {
                    *frame_sample = *sample;
                }
            }
        },
        |err| {
            eprintln!("Stream error: {:?}", err);
        },
        None,
    );

    match stream {
        Ok(s) => {
            println!("Stream created successfully");
            s.play()?; // Play the stream
            Ok(s)
        }
        Err(e) => {
            println!("Failed to create stream");
            Err(Box::new(e))
        }
    }
}
