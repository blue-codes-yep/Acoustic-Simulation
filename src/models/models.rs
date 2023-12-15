// src/models.rs
pub struct SoundWave {
    pub frequency: f32,
    pub amplitude: f32,
    pub direction: f32,
}

pub enum Material {
    Wood {
        absorption_coefficient: f32,
    },
    Metal {
        absorption_coefficient: f32,
    },
    Glass {
        absorption_coefficient: f32,
    },
}