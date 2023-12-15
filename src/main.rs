use sound::sound::generate_wave;
use sound::sound::play_wave;
#[cfg(test)]
mod tests;
mod engine;
mod models;
mod sound;

fn main() {
    // The sample rate and channel count are taken from the default output format
    let sample_rate = 44100.0; // Replace with your desired sample rate

    // The frequency of the sine wave in Hz
    let frequency = 528.0;

    // The volume of the sine wave
    let volume = 0.5;

    let wave = generate_wave(sample_rate, frequency, volume);

    if let Err(e) = play_wave(wave) {
        eprintln!("Error playing wave: {:?}", e);
    }
    
}