use eframe::egui::{DragValue, Ui};

use crate::Mathsim;

impl Mathsim {
    pub(crate) fn camera_options(&mut self, ui: &mut Ui) {
        ui.collapsing("Camera Options", |ui| {
            ui.horizontal(|ui| {
                ui.label("Viewport width : ");
                ui.add(
                    DragValue::new(&mut self.camera.viewport_width)
                        .suffix(" m")
                        .speed(0.1)
                        .range(1..=u64::MAX),
                )
                .changed()
                .then(|| self.camera.viewport_width_updated());
            });
            ui.horizontal(|ui| {
                const MAX_ARROW_SIZE: i32 = 45;
                ui.label("Vector arrow size");
                ui.add(
                    DragValue::new(&mut self.camera.arrow_size)
                        .speed(0.1)
                        .range(0..=MAX_ARROW_SIZE)
                        .suffix(" pt"),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Forces scale: 100 pt =");
                let mut newton_scale = 100. / self.camera.newton_scale;
                ui.add(
                    DragValue::new(&mut newton_scale)
                        .range(0.01..=100000.)
                        .suffix(" N")
                        .speed(1),
                )
                .changed()
                .then(|| self.camera.newton_scale = 100. / newton_scale);
            });
            ui.horizontal(|ui| {
                ui.label("Velocity scale: 100 pt =");
                let mut vel = 100. / self.camera.velocity_scale;
                ui.add(
                    DragValue::new(&mut vel)
                        .range(0.01..=10000.)
                        .suffix(" m/s")
                        .speed(1),
                )
                .changed()
                .then(|| self.camera.velocity_scale = 100. / vel);
            });

            ui.checkbox(&mut self.light_theme, "Light theme 💡");
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.camera.display_forces, "Display forces");
                ui.color_edit_button_srgba(&mut self.camera.forces_color);
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.camera.display_velocity, "Display velocity");
                ui.color_edit_button_srgba(&mut self.camera.velocity_color);
            });
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.camera.display_acceleration,
                    "Display acceleration",
                );
                ui.color_edit_button_srgba(&mut self.camera.acceleration_color);
            });
            ui.horizontal(|ui| {
                ui.checkbox(
                    &mut self.camera.display_last_pos,
                    "Display last point position",
                );
                ui.color_edit_button_srgba(&mut self.camera.last_pos_color);
            });
        });
    }
}
