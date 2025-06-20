use std::process::exit;

use disk_persist::DiskPersist;
use eframe::egui::Context;
use eframe::egui::Window;

use crate::Mathsim;
mod air_drag_options;
mod camera_options;
mod collision_options;
mod gravitational_interaction_options;
mod new_point_options;
mod time_parameters;
mod weight_force_options;

pub(crate) struct SettingsPaneRenderer;

// TODO, dont put the render settings on app
impl SettingsPaneRenderer {
    pub(crate) fn render(app: &mut Mathsim, ctx: &Context) {
        let mut settings_pane_open = app.settings_pane_open;
        Window::new("Settings")
            .open(&mut settings_pane_open)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Actions : ");
                    if ui.button("Reset").clicked() {
                        // Reset the entire app state, clearing every points, settings, etc.
                        *app = Mathsim::default();
                    };
                    if ui.button("Clear balls").clicked() {
                        app.engine.points.clear();
                        app.engine.options.new_points.id_tracker = 0;
                    };
                    if ui.button("Load state").clicked() {
                        let persist: DiskPersist<Mathsim> = DiskPersist::init("mathsim").unwrap();
                        *app = persist.read().unwrap_or_default().unwrap_or_default();
                    }
                    if ui.button("Save state").clicked() {
                        let persist: DiskPersist<Mathsim> = DiskPersist::init("mathsim").unwrap();
                        persist.write(&app).unwrap();
                    }
                    if ui.button("Quit").clicked() {
                        exit(0);
                    }
                });

                ui.separator();

                app.camera_options(ui);
                app.engine.options.weight_force.render_settings(ui);
                app.engine.options.gravity.render_settings(ui);
                app.engine
                    .options
                    .collisions
                    .render_settings(&mut app.camera, ui);
                app.engine.options.air_drag.render_settings(ui);
                app.engine.options.new_points.render_settings(ui);
                app.engine.options.time.render_settings(ui);
            });
        app.settings_pane_open = settings_pane_open;
    }
}
