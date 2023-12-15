use crate::engine::engine::interact;
use crate::models::models::{SoundWave, Material};


#[test]fn test_interact() {
    let wave = SoundWave {
        frequency: 440.0,
        amplitude: 1.0,
        direction: 0.0,
    };
    let material = Material::Wood { absorption_coefficient: 0.5 };
    let result = interact(&wave, &material);
    assert!(result.amplitude < wave.amplitude); // Test that some absorption occurred

    if result.amplitude < wave.amplitude {
        println!("Test passed. The amplitude after interaction ({}) is less than the original amplitude ({}).", result.amplitude, wave.amplitude);
    } else {
        println!("Test failed. The amplitude after interaction ({}) is not less than the original amplitude ({}).", result.amplitude, wave.amplitude);
    }
}