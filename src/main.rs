// main.rs

use sound::sound::{generate_wave, play_wave};
use models::models::{SoundWave, Material};
#[cfg(test)]
mod tests;
mod engine;
mod models;
mod sound;
use crate::models::models::Engine;
use std::thread;
use rapier3d::math::Vector;

fn main() {
    // Initialize the physics engine
    let mut engine = Engine::new();

    // Define the simulation area
    // Example: Creating a simple room with walls and a floor
    // Note: You need to replace these with actual calls to create and add these objects in Rapier3D
    engine.create_room(10.0, 10.0, 10.0); // Room dimensions (width, height, depth)

    // Generate a sound wave
    let sample_rate = 44100.0;
    let frequency = 528.0;
    // ...
    let mut wave: Vec<f32> = generate_wave(44100.0, 528.0, 0.5, Vector::new(0.0, 0.0, 0.0));
    // ...

    // Open the physics engine in a GUI
    // engine.open_gui();

    // Play the sound wave
    match play_wave(wave.to_vec()) {
        Ok(_) => println!("Sound played successfully."),
        Err(e) => eprintln!("Error playing sound: {:?}", e),
    }

    // Sleep to allow sound playback
    std::thread::sleep(std::time::Duration::from_secs(30));
}