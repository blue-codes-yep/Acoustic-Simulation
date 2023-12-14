use crate::engine::engine::interact;
use crate::models::models::{SoundWave, Material};

#[test]
fn test_interact() {
    let wave = SoundWave {
        frequency: 440.0,
        amplitude: 1.0,
        direction: 0.0,
    };
    let material = Material::Wood;
    let result = interact(&wave, &material);
    assert!(result.amplitude < wave.amplitude); // Test that some absorption occurred
}