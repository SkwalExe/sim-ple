use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct TimeOptions {
    pub(crate) substeps: usize,
    pub(crate) time_scale: f32,
    pub(crate) fixed_dt: f32,
    pub(crate) use_fixed_dt: bool,
    pub(crate) paused: bool,
    pub(crate) temporary_unpause: bool,
    pub(crate) limit_fps: bool,
    pub(crate) max_fps: u64,
}

const DEFAULT_USE_FIXED_DT: bool = false;
const DEFAULT_FIXED_DT: f32 = 0.01;
const DEFAULT_PAUSED: bool = false;
const DEFAULT_TEMPORARY_UNPAUSE: bool = false;
const DEFAULT_SUBSTEPS: usize = 1;
const DEFAULT_TIME_SCALE: f32 = 1.;
const DEFAULT_LIMIT_FPS: bool = false;
const DEFAULT_MAX_FPS: u64 = 60;

impl Default for TimeOptions {
    fn default() -> Self {
        Self {
            substeps: DEFAULT_SUBSTEPS,
            use_fixed_dt: DEFAULT_USE_FIXED_DT,
            fixed_dt: DEFAULT_FIXED_DT,
            time_scale: DEFAULT_TIME_SCALE,
            temporary_unpause: DEFAULT_TEMPORARY_UNPAUSE,
            paused: DEFAULT_PAUSED,
            limit_fps: DEFAULT_LIMIT_FPS,
            max_fps: DEFAULT_MAX_FPS,
        }
    }
}
