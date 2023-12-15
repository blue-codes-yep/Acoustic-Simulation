use crate::models::models::{Engine, Material, SoundWave};
use rapier3d::dynamics::{
    CCDSolver, ImpulseJoint, IntegrationParameters, MultibodyJointSet, RigidBodySet,
};
use rapier3d::geometry::{BroadPhase, NarrowPhase};
use rapier3d::math::Vector;
use rapier3d::prelude::QueryPipeline;
use rapier3d::prelude::*;
use std::collections::HashMap;
// Function to interact sound waves with materials
pub fn interact(wave: &SoundWave, material: &Material) -> SoundWave {
    impl Material {
        fn absorption_coefficient(&self) -> f32 {
            match self {
                Material::Wood { absorption_coefficient } => *absorption_coefficient,
                Material::Metal { absorption_coefficient } => *absorption_coefficient,
                Material::Glass { absorption_coefficient } => *absorption_coefficient,
            }
        }
    }
    let absorption_coefficient = material.absorption_coefficient();
    let reflection_coefficient = 1.0 - absorption_coefficient;

    // Calculate the reflected amplitude
    let reflected_amplitude = wave.amplitude * reflection_coefficient;

    // Create a new sound wave with the reflected amplitude
    SoundWave {
        frequency: wave.frequency,
        amplitude: reflected_amplitude,
        direction: wave.direction,
        position: wave.position,
    }
}


// Implementation of Engine struct for physics simulation
impl Engine {
    pub fn interact_mapping(&self, wave: &SoundWave, material: &Material) -> SoundWave {
        // Determine the absorption coefficient of the material
        let absorption_coefficient = material.absorption_coefficient();

        // Calculate the reflected amplitude
        let reflected_amplitude = wave.amplitude * (1.0 - absorption_coefficient);

        // Create a new sound wave with the reflected amplitude
        SoundWave {
            frequency: wave.frequency,
            amplitude: reflected_amplitude,
            direction: wave.direction,
            position: wave.position,
        }
    }
    pub fn new() -> Self {
        let gravity = Vector::new(0.0, -9.81, 0.0);
        let physics = PhysicsPipeline::new();
        let integration_parameters = IntegrationParameters::default();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let bodies = RigidBodySet::new();
        let colliders = ColliderSet::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let material_mapping = HashMap::new();
        let sound_mapping = HashMap::new();
        let interaction_mapping = HashMap::new();
        Self {
            gravity,
            physics,
            integration_parameters,
            broad_phase,
            narrow_phase,
            bodies,
            colliders,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            material_mapping,
            sound_mapping,
            interaction_mapping,
        }
    }

    pub fn step(&mut self) {
        let mut islands = IslandManager::new();

        self.physics.step(
            &self.gravity,
            &self.integration_parameters,
            &mut islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );
    }

    pub fn create_room(&mut self, width: f32, height: f32, depth: f32) {
        // Create floor
        let floor = ColliderBuilder::cuboid(width / 2.0, 0.1, depth / 2.0).build();
        self.colliders.insert(floor);

        // Create walls
        // Front wall
        let front_wall = ColliderBuilder::cuboid(width / 2.0, height / 2.0, 0.1)
            .translation(Vector::new(0.0, height / 2.0, depth / 2.0))
            .build();
        self.colliders.insert(front_wall);

        // Back wall
        let back_wall = ColliderBuilder::cuboid(width / 2.0, height / 2.0, 0.1)
            .translation(Vector::new(0.0, height / 2.0, -depth / 2.0))
            .build();
        self.colliders.insert(back_wall);

        // Left wall
        let left_wall = ColliderBuilder::cuboid(0.1, height / 2.0, depth / 2.0)
            .translation(Vector::new(-width / 2.0, height / 2.0, 0.0))
            .build();
        self.colliders.insert(left_wall);

        // Right wall
        let right_wall = ColliderBuilder::cuboid(0.1, height / 2.0, depth / 2.0)
            .translation(Vector::new(width / 2.0, height / 2.0, 0.0))
            .build();
        self.colliders.insert(right_wall);
    }

    pub fn process_sound_wave_interactions(&mut self, wave: &SoundWave) {
        for (body_handle, body) in self.bodies.iter() {
            if let Some(material) = self.get_material_of_body(body_handle) {
                let interacted_wave = interact(wave, &material);
                // Process interaction
            }
        }
    }

   pub fn get_material_of_body(&self, body_handle: RigidBodyHandle) -> Option<Material> {
        self.material_mapping.get(&body_handle).cloned()
    }
}



    

