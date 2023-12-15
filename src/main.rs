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

    let stream_result = play_wave(wave);

    match stream_result {
        Ok(_stream) => {
            // Sleep for a while so we can hear the sound
            std::thread::sleep(std::time::Duration::from_secs(2));
        },
        Err(e) => {
            eprintln!("Error playing wave: {:?}", e);
        },
    }
}