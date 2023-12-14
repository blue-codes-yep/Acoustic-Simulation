use crate::models::models::{SoundWave, Material};

pub fn interact(wave: &SoundWave, material: &Material) -> SoundWave {
    // Implement the interaction algorithm here
    // For now, return a placeholder sound wave
    SoundWave {
        frequency: wave.frequency,
        amplitude: wave.amplitude * 0.5, // Assume some absorption
        direction: wave.direction,
    }
}