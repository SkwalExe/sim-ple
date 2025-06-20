use super::Point;
use glam::Vec2;

impl Point {
    /// Move the point according to all the applied forces and the current velocity using Verlet integration.
    /// This has to be called after adding every forces. Will actually change the position of the
    /// point.
    pub(crate) fn verlet(&mut self) {
        // Never move if the point is pinned.
        if self.pinned {
            // We set the last pos to make the velocity null, which is the case.
            self.last_pos = self.pos;
            return;
        }
        // Cannot set self.pos directly because we need to save it to self.last_pos. Cannot set
        // self.last_pos before because it would null self.vel().
        // Verlet integration formula: Pn+1 = Pn + vt + 1/2 at^2.
        let next_pos = self.pos + self.dt * self.vel() + self.dt.powi(2) * self.acceleration() / 2.;
        self.last_pos = self.pos;
        self.pos = next_pos;
    }

    /// Add a gravitational interaction force towards each other point.
    /// TODO: we should implement pairwise gravity. i.e. add the force to the two points
    /// simultaneously.
    pub(crate) fn gravity(&mut self, points: &Vec<Point>, gravitational_constant: f32) {
        for b in points {
            // Don't make the point attracted by itself.
            if b.id == self.id {
                continue;
            }
            // The vector from self to the other point (B).
            let ab = b.pos - self.pos;
            // `gravitational_constant` is usually G=6.67e-11.
            // Formula for gravitational interaction:
            // Fa->b = G ma*mb / AB^2
            let force = gravitational_constant * self.mass * b.mass / ab.length_squared();
            // Our force is in the same direction than A->B.
            let force_vector = ab.normalize() * force;
            if force_vector.is_nan() {
                eprintln!(
                    "Skipped gravity force for this frame, because the system would explode."
                );
                return;
            }
            self.forces.push(force_vector);
        }
    }

    /// Adds a quadratic drag force, opposite to the velocity.
    /// The ball will slow down a lot faster then with friction_linear
    /// at high speed.
    pub(crate) fn friction_quadratic(&mut self, drag: f32) {
        let vel = self.vel();
        // This is not at all an exact formula, not even an approximation.
        // It just looks natural, and is computationally accessible.
        // Formula: Fd = Cv^2
        // where C is a drag coefficient and v the current velocity (not the vector).
        self.forces.push(-drag * vel * vel.length())
    }

    /// Adds a linear drag force, opposite to the velocity.
    pub(crate) fn friction_linear(&mut self, drag: f32) {
        let vel = self.vel();
        // This is not at all an exact formula, not even an approximation.
        // It just looks natural, and is computationally accessible.
        // Formula: Fd = Cv
        // where C is a drag coefficient and v the current velocity.
        self.forces.push(-drag * vel)
    }

    /// Adds a weight force to the point.
    pub(crate) fn weight_force(&mut self, strength: Vec2) {
        // Formula: P-> = mg->
        self.forces.push(strength * self.mass);
    }

    /// Computes the potantial energy in Joules
    /// TODO: not sure this is stable.
    pub(crate) fn potential_energy(&self, g: Vec2, ground: Vec2) -> f32 {
        self.mass * (g.y * (ground.y - self.pos.y) + g.x * (ground.x - self.pos.x))
    }

    /// Computes the kinetic energy in Joules.
    pub(crate) fn kinetic_energy(&self) -> f32 {
        // formuma: KE = 1/2 m v^2
        self.mass * self.vel().length_squared() / 2.
    }

    /// Returns the vectorial sum of all forces.
    pub(crate) fn forces_vectorial_sum(&self) -> Vec2 {
        self.forces.iter().sum::<Vec2>()
    }

    /// Returns the acceleration in m.s^{-2}
    pub(crate) fn acceleration(&self) -> Vec2 {
        // Newton's second law of motion:
        // F = ma
        // <=> a = F/m
        // Where F is the vectorial sum of all forces.
        self.forces_vectorial_sum() / self.mass
    }

    /// Returns the displacement vector of the point in the last frame in meters.
    pub(crate) fn displacement(&self) -> Vec2 {
        self.pos - self.last_pos
    }
    /// Returns the velocity in m.s^{-1}
    pub(crate) fn vel(&self) -> Vec2 {
        // Formula: v = d/t
        // where d is the displacement of the point
        // in the last frame.
        self.displacement() / self.last_dt
    }
}
