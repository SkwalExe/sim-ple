use glam::Vec2;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BoundingBox {
    pub(crate) min: Vec2,
    pub(crate) max: Vec2,
}

pub(crate) trait Coord {
    fn coord(&self, coord_i: usize) -> f32;
    fn coord_mut(&mut self, coord_i: usize) -> &mut f32;
}

impl Coord for Vec2 {
    fn coord(&self, coord_i: usize) -> f32 {
        if coord_i == 0 { self.x } else { self.y }
    }
    fn coord_mut(&mut self, coord_i: usize) -> &mut f32 {
        if coord_i == 0 {
            &mut self.x
        } else {
            &mut self.y
        }
    }
}

impl BoundingBox {
    pub(crate) fn move_into_on_axis(&self, other: &BoundingBox, axis: usize) -> Option<f32> {
        let min = other.min.coord(axis);
        let max = other.max.coord(axis);
        let (new_min, new_max) = (min.min(self.min.coord(axis)), max.max(self.max.coord(axis)));
        if new_min != min && new_max != max {
            error!("Min: {min} Max: {max}, Newmin: {new_min}, {new_max}");
            return None;
        }
        Some(if new_min != min {
            min - new_min
        } else if new_max != max {
            max - new_max
        } else {
            0.
        })
    }
    pub(crate) fn move_into(&self, other: &BoundingBox) -> Option<Vec2> {
        Some(Vec2::new(
            self.move_into_on_axis(other, 0)?,
            self.move_into_on_axis(other, 1)?,
        ))
    }
    pub(crate) fn contains_on_axis(&self, other: &BoundingBox, axis: usize) -> bool {
        other.min.coord(axis) >= self.min.coord(axis)
            && other.max.coord(axis) <= self.max.coord(axis)
    }
    pub(crate) fn _contains(&self, other: &BoundingBox) -> bool {
        self.contains_on_axis(other, 0) && self.contains_on_axis(other, 1)
    }
    /// Returns an array containing the 4 points forming the bounding box.
    /// The first point is the top left corner and the order is clockwise
    pub(crate) fn _points(&self) -> Vec<Vec2> {
        vec![
            // Top left
            Vec2::new(self.min.x, self.max.y),
            // Top right
            self.max,
            // bottom right
            Vec2::new(self.max.x, self.min.y),
            // bottom left
            self.min,
        ]
    }
}
