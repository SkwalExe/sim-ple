use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct WeightForceOptions {
    pub(crate) enabled: bool,
    pub(crate) intensity: Vec2,
}

const DEFAULT_WEIGHT_FORCE_ENABLED: bool = true;
const DEFAULT_WEIGHT_INTENSITY: Vec2 = Vec2::new(0., -9.81);

impl Default for WeightForceOptions {
    fn default() -> Self {
        Self {
            enabled: DEFAULT_WEIGHT_FORCE_ENABLED,
            intensity: DEFAULT_WEIGHT_INTENSITY,
        }
    }
}
