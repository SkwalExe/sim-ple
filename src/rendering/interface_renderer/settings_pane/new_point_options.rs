use eframe::egui::{DragValue, Ui};

use crate::physics::{
    engine::simulation_parameters::new_point_options::NewPointOptions, objects::Point,
};

impl NewPointOptions {
    pub(crate) fn render_settings(&mut self, ui: &mut Ui) {
        ui.collapsing("New points", |ui| {
            ui.horizontal(|ui| {
                ui.label("Balls to add per click : ");
                ui.add(
                    DragValue::new(&mut self.balls_per_click)
                        .speed(0.1)
                        .range(1..=u64::MAX),
                );
            });
            ui.horizontal(|ui| {
                ui.label("New points:");
                if ui.button("Reset").clicked() {
                    self.default_point = Point::default();
                }
            });
            ui.checkbox(&mut self.default_point.pinned, "Pin point.");
            ui.horizontal(|ui| {
                ui.label("Mass : ");
                ui.add(
                    DragValue::new(&mut self.default_point.mass)
                        .speed(0.1)
                        .range(0.001..=f32::MAX)
                        .suffix(" kg"),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Radius : ");
                ui.add(
                    DragValue::new(&mut self.default_point.radius)
                        .speed(0.01)
                        .range(0.001..=f32::MAX)
                        .suffix(" m"),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Color : ");
                ui.color_edit_button_srgba(&mut self.default_point.color);
            });
        });
    }
}
