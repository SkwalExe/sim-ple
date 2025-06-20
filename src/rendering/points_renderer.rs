use eframe::egui::{Shape, Stroke, Ui};
use egui::{Color32, Pos2};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::physics::{Engine, objects::Point};

use super::{Camera, vector_renderer::VecRenderer};

pub(crate) struct PointsRenderer;

impl PointsRenderer {
    pub(crate) fn render(camera: &Camera, ui: &mut Ui, engine: &Engine) {
        let painter = ui.painter();
        let mut shapes: Vec<Shape> = Vec::new();

        if camera.display_last_pos {
            shapes.extend(Self::render_last_pos(&engine.points, camera));
        }

        if camera.display_velocity {
            shapes.extend(Self::display_velocity(&engine.points, camera));
        }

        if camera.display_forces {
            shapes.extend(Self::display_forces(&engine.points, camera));
        }

        if camera.display_acceleration {
            shapes.extend(Self::display_acceleration(&engine.points, camera));
        }

        shapes.extend(Self::render_trackers(&engine.points, camera));

        // The points themselves
        let circles = engine
            .points
            .par_iter()
            .map(|p| {
                Shape::circle_filled(
                    camera.world_to_screen(p.pos),
                    camera.len_to_screen(p.radius),
                    p.color,
                )
            })
            .collect::<Vec<Shape>>();
        shapes.extend(circles);

        // Draw all the shapges.
        painter.extend(shapes);
    }

    pub(crate) fn render_trackers(points: &Vec<Point>, camera: &Camera) -> Vec<Shape> {
        points
            .par_iter()
            .filter(|p| p.tracked)
            .flat_map(|p| {
                let pos = camera.world_to_screen(p.pos);
                vec![
                    Shape::line_segment(
                        [
                            Pos2::new(pos.x, camera.screen_viewport.min.y),
                            Pos2::new(pos.x, camera.screen_viewport.max.y),
                        ],
                        Stroke::new(2., Color32::RED),
                    ),
                    Shape::line_segment(
                        [
                            Pos2::new(camera.screen_viewport.min.x, pos.y),
                            Pos2::new(camera.screen_viewport.max.x, pos.y),
                        ],
                        Stroke::new(2., Color32::RED),
                    ),
                ]
            })
            .collect()
    }

    pub(crate) fn display_velocity(points: &Vec<Point>, camera: &Camera) -> Vec<Shape> {
        points
            .par_iter()
            .flat_map(|p| {
                VecRenderer::get_shapes(
                    camera.world_to_screen(p.pos),
                    camera.world_to_screen(p.pos + p.vel() * camera.velocity_scale),
                    Stroke::new(2., camera.velocity_color),
                    camera.arrow_size,
                )
            })
            .collect()
    }
    pub(crate) fn display_acceleration(points: &Vec<Point>, camera: &Camera) -> Vec<Shape> {
        points
            .par_iter()
            .flat_map(|p| {
                VecRenderer::get_shapes(
                    camera.world_to_screen(p.pos),
                    camera.world_to_screen(p.pos + p.acceleration() * camera.velocity_scale),
                    Stroke::new(2., camera.acceleration_color),
                    camera.arrow_size,
                )
            })
            .collect()
    }
    pub(crate) fn display_forces(points: &Vec<Point>, camera: &Camera) -> Vec<Shape> {
        points
            .par_iter()
            .flat_map(|p| {
                p.forces
                    .iter()
                    .flat_map(|force| {
                        VecRenderer::get_shapes(
                            camera.world_to_screen(p.pos),
                            camera.world_to_screen(p.pos + force * camera.newton_scale),
                            Stroke::new(2., camera.forces_color),
                            camera.arrow_size,
                        )
                    })
                    .collect::<Vec<Shape>>()
            })
            .collect()
    }

    pub(crate) fn render_last_pos(points: &Vec<Point>, camera: &Camera) -> Vec<Shape> {
        points
            .par_iter()
            .map(|p| {
                Shape::circle_filled(
                    camera.world_to_screen(p.last_pos),
                    camera.len_to_screen(p.radius),
                    camera.last_pos_color,
                )
            })
            .collect()
    }
}
