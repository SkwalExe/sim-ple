use eframe::egui::{DragValue, Ui, UiBuilder};

use crate::physics::engine::simulation_parameters::gravity_options::GravityOptions;

impl GravityOptions {
    pub(crate) fn render_settings(&mut self, ui: &mut Ui) {
        ui.collapsing("Gravitational interaction", |ui| {
            ui.checkbox(&mut self.enabled, "Gravitational interaction");
            let mut gravity_settings = UiBuilder::new();
            gravity_settings.disabled = !self.enabled;
            ui.scope_builder(gravity_settings, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Gravitational constant: ");
                    ui.add(DragValue::new(&mut self.gravitational_constant).speed(0.1))
                });
            });
        });
    }
}
