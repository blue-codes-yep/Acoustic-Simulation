// src/models.rs
use rapier3d::prelude::*;
use std::collections::HashMap;

pub struct SoundWave {
    pub frequency: f32,
    pub amplitude: f32,
    pub direction: f32,
}

#[derive(Clone)]
pub enum Material {
    Wood { absorption_coefficient: f32 },
    Metal { absorption_coefficient: f32 },
    Glass { absorption_coefficient: f32 },

}

pub struct Engine {
    pub gravity: Vector<f32>,
    pub physics: PhysicsPipeline,
    pub integration_parameters: IntegrationParameters,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub query_pipeline: QueryPipeline,
    pub ccd_solver: CCDSolver,
    pub material_mapping: HashMap<RigidBodyHandle, Material>,
    pub sound_mapping: HashMap<RigidBodyHandle, SoundWave>,
    pub interaction_mapping: HashMap<RigidBodyHandle, Vec<RigidBodyHandle>>,
}