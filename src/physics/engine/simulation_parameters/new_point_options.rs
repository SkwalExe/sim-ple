use serde::{Deserialize, Serialize};

use crate::physics::objects::Point;

#[derive(Serialize, Deserialize)]
pub(crate) struct NewPointOptions {
    /// The default state used for the new points.
    pub(crate) default_point: Point,
    #[serde(skip)]
    pub(crate) id_tracker: i32,
    pub(crate) balls_per_click: usize,
}

const DEFAULT_BALLS_PER_CLICK: usize = 1;

impl Default for NewPointOptions {
    fn default() -> Self {
        Self {
            default_point: Point::default(),
            id_tracker: 0,
            balls_per_click: DEFAULT_BALLS_PER_CLICK,
        }
    }
}
