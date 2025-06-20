use std::fmt::Debug;

use eframe::egui::Color32;
use glam::Vec2;
use serde::{Deserialize, Serialize};
mod collisions;
mod physics;
mod utils;

const DEFAULT_POINT_COLOR: Color32 = Color32::from_rgba_premultiplied(133, 107, 183, 150);
const POINT_PINNED_DEFAULT: bool = false;
const POINT_RADIUS_DEFAULT: f32 = 0.1;
const POINT_MASS_DEFAULT: f32 = 1.;
const DEFAULT_POINT_TRACKED: bool = false;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Point {
    /// The position of the point in the current frame, in meters.
    pub(crate) pos: Vec2,
    /// The position of the point at the end of the last frame.
    pub(crate) last_pos: Vec2,
    /// The radius of the point in meters
    pub(crate) radius: f32,
    /// The mass of the point in kgs
    pub(crate) mass: f32,
    /// The time it took to render the last frame. Saved here for easier access.
    /// /!\ But we are considering this as the time it will take to render the current frame.
    pub(crate) dt: f32,
    /// The delta time used in the previous frame.
    pub(crate) last_dt: f32,
    /// The forces applied to the point in Newtons.
    /// This value is nulled at the start of every frame.
    pub(crate) forces: Vec<Vec2>,
    /// The unique identifier of this point, used to prevent colliding with itself, etc.
    pub(crate) id: i32,
    /// The rendered color of this point.
    pub(crate) color: Color32,
    /// If true, the point should never move, and we can ignore all forces.
    pub(crate) pinned: bool,
    /// If true, the point will be highlighed
    pub(crate) tracked: bool,
}

impl Debug for Point {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       f.debug_struct("Point")
            .field("pos", &self.pos)
            .field("acc", &self.acceleration())
            .field("displacement", &self.displacement())
            .field("vel", &self.vel())
            .finish()
   } 
}

impl Default for Point {
    fn default() -> Self {
        Self {
            // The ID is set when the point is created
            id: 0,
            // The initial value of delta time doesn't matter as long as it isn't 0.
            last_dt: 1.,
            dt: 1.,
            // The original position doesn't matter
            // because it is always set when added to the scene.
            pos: Default::default(),
            // The last pos is also set when the point is added to the scene.
            last_pos: Default::default(),
            // The forces list is cleared at the beginning of every frame.
            forces: Vec::new(),
            // These are good defaults, and can be changed by the user
            // before adding the point to the scene.
            pinned: POINT_PINNED_DEFAULT,
            radius: POINT_RADIUS_DEFAULT,
            mass: POINT_MASS_DEFAULT,
            color: DEFAULT_POINT_COLOR,
            tracked: DEFAULT_POINT_TRACKED,
        }
    }
}
