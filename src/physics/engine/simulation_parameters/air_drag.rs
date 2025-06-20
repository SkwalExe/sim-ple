use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct AirDragOptions {
    pub(crate) enabled: bool,
    /// Whether to use a quadratic or linear friction
    pub(crate) quadradic: bool,
    pub(crate) friction_intensity: f32,
}

const DEFAULT_AIR_DRAG_ENABLED: bool = true;
const DEFAULT_FRICTION_INTENSITY: f32 = 0.5;
const DEFAULT_USE_QUADRADIC_FRICTION: bool = false;

impl Default for AirDragOptions {
    fn default() -> Self {
        Self {
            enabled: DEFAULT_AIR_DRAG_ENABLED,
            quadradic: DEFAULT_USE_QUADRADIC_FRICTION,
            friction_intensity: DEFAULT_FRICTION_INTENSITY,
        }
    }
}
