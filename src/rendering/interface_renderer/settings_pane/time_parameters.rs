use eframe::egui::{DragValue, Ui, UiBuilder};

use crate::physics::engine::simulation_parameters::time_parameters::TimeOptions;

impl TimeOptions {
    pub(crate) fn render_settings(&mut self, ui: &mut Ui) {
        ui.collapsing("Time parameters", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.button(if self.paused { "Unpause" } else { "Pause" })
                    .clicked()
                    .then(|| self.paused = !self.paused);
                ui.button("Next frame")
                    .clicked()
                    .then(|| self.temporary_unpause = true);
            });
            ui.checkbox(&mut self.limit_fps, "Limit fps");
            ui.horizontal(|ui| {
                ui.label("FPS limit:");
                ui.add(DragValue::new(&mut self.max_fps).speed(0.1).range(10..=512));
            });
            ui.horizontal(|ui| {
                ui.label("Time acceleration: ");
                ui.add(
                    DragValue::new(&mut self.time_scale)
                        .prefix("x")
                        .speed(0.01)
                        .range(0.001..=10.),
                )
            });
            ui.horizontal(|ui| {
                ui.label("Substeps");
                ui.add(DragValue::new(&mut self.substeps).speed(0.1).range(1..=10));
            });
            ui.checkbox(&mut self.use_fixed_dt, "Use fixed delta_time");
            let mut fixed_dt_settings = UiBuilder::new();
            fixed_dt_settings.disabled = !self.use_fixed_dt;
            ui.scope_builder(fixed_dt_settings, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Fixed delta time: ");
                    let mut fixed_dt = self.fixed_dt * 1000.;
                    ui.add(DragValue::new(&mut fixed_dt).suffix(" ms").range(1..=10000))
                        .changed()
                        .then(|| {
                            self.fixed_dt = fixed_dt / 1000.;
                        });
                });
            });
        });
    }
}
