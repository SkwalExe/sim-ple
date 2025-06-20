use eframe::egui::{DragValue, Ui, UiBuilder};

use crate::physics::engine::simulation_parameters::weight_force_options::WeightForceOptions;

impl WeightForceOptions {
    pub(crate) fn render_settings(&mut self, ui: &mut Ui) {
        ui.collapsing("Weight force", |ui| {
            ui.checkbox(&mut self.enabled, "Enable weight force.");
            let mut weight_force_settings = UiBuilder::new();
            weight_force_settings.disabled = !self.enabled;
            ui.scope_builder(weight_force_settings, |ui| {
                ui.label("Gravitational field strength (weight force)");
                ui.horizontal(|ui| {
                    ui.label("x : ");
                    ui.add(
                        DragValue::new(&mut self.intensity.x)
                            .speed(0.1)
                            .suffix(" N/kg"),
                    );
                    ui.label("y : ");
                    ui.add(
                        DragValue::new(&mut self.intensity.y)
                            .speed(0.1)
                            .suffix(" N/kg"),
                    );
                });
            });
        });
    }
}
