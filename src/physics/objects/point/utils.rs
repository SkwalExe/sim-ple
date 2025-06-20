use glam::Vec2;

use crate::physics::BoundingBox;

use super::Point;
impl Point {
    /// Setups the point for the next frame. This should be called before anything.
    /// Sets dt and reset forces.
    pub(crate) fn setup(&mut self, dt: f32) {
        self.forces.clear();
        self.last_dt = self.dt;
        self.dt = dt;
    }

    /// Consumes the current instance and return a new one with the ID updated.
    pub(crate) fn with_id(mut self, id: i32) -> Self {
        self.set_id(id);
        self
    }

    /// Consumes the current instance and return a new one with the velocity updated.
    pub(crate) fn with_vel(mut self, vel: Vec2) -> Self {
        self.set_vel(vel);
        self
    }

    /// Returns false if a crucial parameter is infinite or Nan
    pub(crate) fn is_normal(&self) -> bool {
        self.pos.is_finite()
    }

    /// Consumes the current point and return a new one with the position updated.
    /// This methods conserves the velocity.
    pub(crate) fn with_pos(mut self, pos: Vec2) -> Self {
        self.set_pos(pos);
        self
    }

    /// Changes the ID of this point.
    pub(crate) fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    /// Changes the velocity, given in meters per second, of this point.
    pub(crate) fn set_vel(&mut self, vel: Vec2) {
        self.set_displacement(vel);
        self.last_dt = 1.;
    }

    /// Changes the last position to match the given displacement.
    pub(crate) fn set_displacement(&mut self, d: Vec2) {
        self.last_pos = self.pos - d;
    }

    /// Changes the position of the point, while keeping the same velocity.
    pub(crate) fn set_pos(&mut self, pos: Vec2) {
        // Keep the same displacement (i.e. also transform the last position).
        let displacement = self.displacement();
        self.pos = pos;
        self.set_displacement(displacement);
    }

    /// Returns the bounding box of the point at the given position.
    pub(crate) fn bounding_box_at(&self, pos: Vec2) -> BoundingBox {
        BoundingBox {
            min: pos - Vec2::splat(self.radius),
            max: pos + Vec2::splat(self.radius),
        }
    }
    /// Returns the bounding box of the point at the current position.
    pub(crate) fn current_bounding_box(&self) -> BoundingBox {
        self.bounding_box_at(self.pos)
    }
}
