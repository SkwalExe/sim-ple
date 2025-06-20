use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct GravityOptions {
    pub(crate) enabled: bool,
    pub(crate) gravitational_constant: f32,
}

const DEFAULT_GRAVITY_ENABLED: bool = false;
const DEFAULT_GRAVITATIONAL_CONSTANT: f32 = 1.;
// const DEFAULT_GRAVITATIONAL_CONSTANT: f32 = 6.67e-11;

impl Default for GravityOptions {
    fn default() -> Self {
        Self {
            enabled: DEFAULT_GRAVITY_ENABLED,
            gravitational_constant: DEFAULT_GRAVITATIONAL_CONSTANT,
        }
    }
}
