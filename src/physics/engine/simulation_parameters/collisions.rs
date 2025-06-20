use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub(crate) enum Container {
    None,
    Rect,
    Circle,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CollisionOptions {
    pub(crate) solver_iterations: usize,
    pub(crate) energy_conservation_perc: f32,
    pub(crate) ball_collisions: bool,
    pub(crate) container_options: ContainerOptions,
}

const DEFAULT_COLLISION_SOLVER_ITERATIONS: usize = 1;
const DEFAULT_ENERGY_CONSERVATION_PERC: f32 = 90.;
const DEFAULT_CONTAINER: Container = Container::Rect;
const DEFAULT_CIRCLE_CONTAINER_RADIUS_PERC: f32 = 45.;
const DEFAULT_BALL_COLLISIONS: bool = true;

impl Default for CollisionOptions {
    fn default() -> Self {
        Self {
            solver_iterations: DEFAULT_COLLISION_SOLVER_ITERATIONS,
            energy_conservation_perc: DEFAULT_ENERGY_CONSERVATION_PERC,
            ball_collisions: DEFAULT_BALL_COLLISIONS,
            container_options: ContainerOptions::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ContainerOptions {
    pub(crate) container: Container,
    pub(crate) circle_container_radius_perc: f32,
}

impl Default for ContainerOptions {
    fn default() -> Self {
        Self {
            container: DEFAULT_CONTAINER,
            circle_container_radius_perc: DEFAULT_CIRCLE_CONTAINER_RADIUS_PERC,
        }
    }
}
