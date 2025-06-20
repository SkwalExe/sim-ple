use glam::Vec2;
use log::info;
use serde::{Deserialize, Serialize};
pub(crate) mod simulation_parameters;

use super::{BoundingBox, objects::Point};

mod options_def;
pub(crate) use options_def::EngineOptions;
mod update;

#[derive(Serialize, Deserialize)]
pub(crate) struct Engine {
    /// All the points currently present in the simulation.
    #[serde(skip)]
    pub(crate) points: Vec<Point>,
    /// The viewport of the camera in world coordinates.
    #[serde(skip)]
    pub(crate) viewport: BoundingBox,
    /// The dt of the last frame.
    #[serde(skip)]
    pub(crate) dt: f32,
    /// The user-set options
    pub(crate) options: EngineOptions,
}

impl Engine {
    pub(crate) fn total_potential_energy(&self) -> f32 {
        let ground = Vec2::new(
            if self.options.weight_force.intensity.x.is_sign_positive() {
                self.viewport.max.x
            } else {
                self.viewport.min.x
            },
            if self.options.weight_force.intensity.y.is_sign_positive() {
                self.viewport.max.y
            } else {
                self.viewport.min.y
            },
        );

        self.points.iter().fold(0., |acc, p| {
            acc + p.potential_energy(self.options.weight_force.intensity, ground)
        })
    }
    pub(crate) fn total_kinetic_energy(&self) -> f32 {
        self.points
            .iter()
            .fold(0., |acc, p| acc + p.kinetic_energy())
    }
    pub(crate) fn add_point(&mut self, point: Point) {
        for _ in 0..self.options.new_points.balls_per_click {
            info!(
                "New point ({}) added at: w{}",
                self.options.new_points.id_tracker, point.pos
            );
            self.points
                .push(point.clone().with_id(self.options.new_points.id_tracker));
            self.options.new_points.id_tracker += 1;
        }
    }
    pub(crate) fn add_default_point(&mut self, pos: Vec2) {
        self.add_point(self.options.new_points.default_point.clone().with_pos(pos));
    }
}

// TODO: refactor the defaults here.
impl Default for Engine {
    fn default() -> Self {
        Self {
            points: vec![],
            // The viewport is set before each frame.
            viewport: Default::default(),
            dt: Default::default(),
            options: EngineOptions::default(),
        }
    }
}
