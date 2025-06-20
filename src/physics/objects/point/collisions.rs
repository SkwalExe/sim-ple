use glam::Vec2;
use rand::random;

use super::Point;

impl Point {
    pub(crate) fn collision(&mut self, points: &Vec<Point>, conservation: f32) {
        for b in points {
            // ======================= DETERMINE IF WE ARE COLLIDING.
            // Avoid making the point collide with itself, and don't move the point if pinned.
            if b.id == self.id || self.pinned || !self.is_normal() || !b.is_normal() {
                continue;
            }

            // The vector from A to B
            // which is normal to the circles.
            let ab = b.pos - self.pos;
            let distance_centers_squared = ab.length_squared();
            // The sum of the two radiuses.
            let radiuses = self.radius + b.radius;

            // If the two circles perfectly overlap, we need
            // to move the point outside of B.
            if distance_centers_squared == 0. {
                // One of the two point only should move.
                if self.id > b.id {
                    // random() returns a f32 between 0 and 1.
                    let random_direction =
                        Vec2::new(random::<f32>(), random::<f32>()) - Vec2::splat(0.5);
                    // Move the point slightly further so that they do not collide again on the
                    // next frame.
                    self.set_pos(self.pos + random_direction.normalize() * 1.01 * radiuses);
                }
                // Then, the points are not colliding anymore.
                continue;
            }

            // yes, we have to use max(0.).... because of f32 imprecision,
            // distance_centers_squared can be negative.
            let distance_centers = distance_centers_squared.max(0.).sqrt();

            // The distance of the overlap between the two circles.
            let overlap = radiuses - distance_centers;

            // We are colliding if this overlap exists, i.e. is positive.
            // If the overlap is 0 then we are not colliding.

            if overlap <= 0. {
                continue;
            }


            // From here we are colliding.
            // ====================== MOVING HALF THE OVERLAP DISTANCE

            // Move half of the overlap distance from the other point.
            let ab_unit = ab.normalize();
            let move_away = ab_unit * overlap / 2.;

            // 1
            // self.pos -= move_away;

            // 2 
            self.set_pos(self.pos - move_away);

            // Also compute the position of the point B after he
            // would have moved from the point A.
            let pos_b = b.pos + move_away;
            let last_pos_b = b.last_pos + move_away;

            // ===================== Conservation of momentum.
            // hic jacet lepus...

            // 1
            // continue;

            // If A is already moving away from B, then dont change velocity;
            if self.vel().dot(ab_unit) < 0. {
                continue;
            }

            // The unit vector tangent to the circles at the point of collision
            let tangent_unit = Vec2::new(-ab_unit.y, ab_unit.x);

            // We will modify the displacement instead of the velocity,
            // because it is equivalent and less intensive computationnaly.
            // But we will use the velocity notation for comprehension sake.
            let vel_b = pos_b - last_pos_b;
            let vel_a = self.displacement();

            // We express the velocity of A and B as linear combinations of
            // the normal and tangent units.
            // For example vel_a_normal is the normal component of this linear combination.
            let vel_a_normal = ab_unit.dot(vel_a);
            let vel_a_tangent = tangent_unit.dot(vel_a);
            let vel_b_normal = ab_unit.dot(vel_b);

            // The tangent component of the velocity of A and B do not change.
            // This line is for comprehension sake, and the rust compiler will optimize this.
            let final_vel_a_tangent = vel_a_tangent;

            // The normal component of the velocity after the collision occured.
            // This is the most important line:
            // formula: Van' = Van * (Ma - Mb) + (2 Mb Vbn) / (Ma + Mb)
            // TODO: explain the formula here.
            let final_vel_a_normal = (vel_a_normal * (self.mass - b.mass)
                + 2. * b.mass * vel_b_normal)
                / (self.mass + b.mass);

            // Recompose the final velocity of point A using the normal and tangent components.
            // Also apply energy loss using the `conservation` ratio.
            let final_vel_a =
                conservation * ab_unit * final_vel_a_normal + tangent_unit * final_vel_a_tangent;

            // Again, we are fancying the displacement as if it was the velocity,
            // which gives the exact same result in the end.
            self.set_displacement(final_vel_a);
        }
    }
}
