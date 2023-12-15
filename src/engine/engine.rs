use crate::models::models::{SoundWave, Material};



pub fn _interact(wave: &SoundWave, _material: &Material) -> SoundWave {
    
    let material = Material::Wood { absorption_coefficient: 0.5 };
    
    let absorption_coefficient = match material {
        Material::Wood { absorption_coefficient } => absorption_coefficient,
        Material::Metal { absorption_coefficient } => absorption_coefficient,
        Material::Glass { absorption_coefficient } => absorption_coefficient,
    };

    let reflection_coefficient = 1.0 - absorption_coefficient;

    // Calculate the reflected amplitude
    let reflected_amplitude = wave.amplitude * reflection_coefficient;

    // Create a new sound wave with the reflected amplitude
    SoundWave {
        frequency: wave.frequency,
        amplitude: reflected_amplitude,
        direction: wave.direction,
    }
}