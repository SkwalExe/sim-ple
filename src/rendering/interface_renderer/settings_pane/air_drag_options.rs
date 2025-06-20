use eframe::egui::{DragValue, Ui, UiBuilder};

use crate::physics::engine::simulation_parameters::air_drag::AirDragOptions;

impl AirDragOptions {
    pub(crate) fn render_settings(&mut self, ui: &mut Ui) {
        ui.collapsing("Air resistance", |ui| {
            ui.checkbox(&mut self.enabled, "Air resistane");
            let mut air_friction_settings = UiBuilder::new();
            air_friction_settings.disabled = !self.enabled;
            ui.scope_builder(air_friction_settings, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Friction type");
                    ui.radio_value(&mut self.quadradic, true, "Quadratic");
                    ui.radio_value(&mut self.quadradic, false, "Linear");
                });
                ui.horizontal(|ui| {
                    ui.label("Air friction : ");
                    let mut air_friction_perc = self.friction_intensity * 100.;
                    ui.add(
                        DragValue::new(&mut air_friction_perc)
                            .suffix("%")
                            .speed(0.1)
                            .range(0..=1000),
                    )
                    .changed()
                    .then(|| {
                        self.friction_intensity = air_friction_perc / 100.;
                    });
                });
            })
        });
    }
}
