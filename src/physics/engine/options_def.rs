use serde::{Deserialize, Serialize};

use super::simulation_parameters::{
    air_drag::AirDragOptions, collisions::CollisionOptions, gravity_options::GravityOptions,
    new_point_options::NewPointOptions, time_parameters::TimeOptions,
    weight_force_options::WeightForceOptions,
};

#[derive(Serialize, Deserialize)]
pub(crate) struct EngineOptions {
    pub(crate) new_points: NewPointOptions,
    pub(crate) time: TimeOptions,
    pub(crate) gravity: GravityOptions,
    pub(crate) weight_force: WeightForceOptions,
    pub(crate) collisions: CollisionOptions,
    pub(crate) air_drag: AirDragOptions,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            air_drag: AirDragOptions::default(),
            new_points: NewPointOptions::default(),
            weight_force: WeightForceOptions::default(),
            gravity: GravityOptions::default(),
            time: TimeOptions::default(),
            collisions: CollisionOptions::default(),
        }
    }
}
