use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::physics::bounding_box::Coord;

use super::{Engine, simulation_parameters::collisions::Container};

impl Engine {
    pub(crate) fn update(&mut self, dt: f32) {
        if self.options.time.paused && !self.options.time.temporary_unpause {
            return;
        }
        self.options.time.temporary_unpause = false;

        self.dt = if self.options.time.use_fixed_dt {
            self.options.time.fixed_dt
        } else {
            dt
        } * self.options.time.time_scale
            / self.options.time.substeps as f32;

        for _ in 0..self.options.time.substeps {
            self.update_points();
        }
    }

    pub(crate) fn points_viewport_collision(&mut self) {
        match self.options.collisions.container_options.container {
            Container::None => {}
            Container::Circle => {
                todo!()
            }
            Container::Rect => {
                self.points.par_iter_mut().for_each(|p| {
                    // We perform viewport collision by reflecting the velocity against the wall.
                    // This is an exact computation, not an approximation.
                    let bb = p.current_bounding_box();
                    let last_pos_bb = p.bounding_box_at(p.last_pos);
                    // perform the same steps on each axis separately.
                    for axis in 0..=1 {
                        // if the current pos is invalid:
                        if !self.viewport.contains_on_axis(&bb, axis) {
                            // but the last pos is valid:
                            if self.viewport.contains_on_axis(&last_pos_bb, axis) {
                                // inverse the velocity
                                let mut displacement = p.displacement();
                                *displacement.coord_mut(axis) *=
                                    -1. * self.options.collisions.energy_conservation_perc / 100.;
                                p.set_displacement(displacement);
                            } else {
                                // if the previous pos is also not valid
                                if let Some(transform) = bb.move_into(&self.viewport) {
                                    p.pos += transform;
                                    // p.set_pos(p.pos + transform);
                                }
                            }
                        }
                    }
                });
            }
        }
    }

    pub(crate) fn ball_collisions(&mut self) {
        if self.options.collisions.ball_collisions {
            let points_copy = self.points.clone();
            let conserv = self.options.collisions.energy_conservation_perc / 100.;
            self.points
                .par_iter_mut()
                .for_each(|point| point.collision(&points_copy, conserv));
        }
    }

    /// Remove all points with invalid position
    fn remove_invalid_points(&mut self) {
        self.points.retain(|p| p.is_normal());
    }

    /// Sets dt and reset the forces
    fn setup_points(&mut self) {
        self.points.par_iter_mut().for_each(|point| {
            point.setup(self.dt);
        });
    }

    fn air_friction(&mut self) {
        if self.options.air_drag.enabled {
            if self.options.air_drag.quadradic {
                self.points.par_iter_mut().for_each(|point| {
                    point.friction_quadratic(self.options.air_drag.friction_intensity)
                });
            } else {
                self.points.par_iter_mut().for_each(|point| {
                    point.friction_linear(self.options.air_drag.friction_intensity)
                });
            }
        }
    }

    fn compute_weight(&mut self) {
        if self.options.weight_force.enabled {
            self.points
                .par_iter_mut()
                .for_each(|point| point.weight_force(self.options.weight_force.intensity));
        }
    }

    fn gravitational_interaction(&mut self) {
        let points_copy = self.points.clone();
        if self.options.gravity.enabled {
            self.points.par_iter_mut().for_each(|point| {
                point.gravity(&points_copy, self.options.gravity.gravitational_constant)
            });
        }
    }
    pub(crate) fn collisions(&mut self) {
        for _ in 0..self.options.collisions.solver_iterations {
            self.points_viewport_collision();
            self.ball_collisions();
        }
    }

    pub(crate) fn verlet(&mut self) {
        self.points.par_iter_mut().for_each(|point| {
            point.verlet();
        });
    }
    pub(crate) fn update_points(&mut self) {
        self.remove_invalid_points();
        self.setup_points();
        self.compute_weight();
        self.gravitational_interaction();
        self.air_friction();
        self.collisions();
        self.verlet();
    }
}
