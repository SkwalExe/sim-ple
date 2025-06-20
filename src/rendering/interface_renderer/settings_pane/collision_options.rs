use eframe::egui::{DragValue, Ui, UiBuilder};

use crate::{
    physics::engine::simulation_parameters::collisions::{CollisionOptions, Container},
    rendering::Camera,
};

impl CollisionOptions {
    pub(crate) fn render_settings(&mut self, camera: &mut Camera, ui: &mut Ui) {
        ui.collapsing("Collisions", |ui| {
            ui.collapsing("Viewport collisions", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Container");
                    ui.radio_value(
                        &mut self.container_options.container,
                        Container::None,
                        "None",
                    );
                    ui.radio_value(
                        &mut self.container_options.container,
                        Container::Circle,
                        "Circle",
                    );
                    ui.radio_value(
                        &mut self.container_options.container,
                        Container::Rect,
                        "Rect",
                    );
                });
                let mut circle_settings = UiBuilder::new();
                circle_settings.disabled = self.container_options.container != Container::Circle;
                ui.scope_builder(circle_settings, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Viewport collider radius:");
                        ui.add(
                            DragValue::new(
                                &mut self.container_options.circle_container_radius_perc,
                            )
                            .speed(0.1)
                            .suffix(" %")
                            .range(1..=45),
                        )
                    });
                });
                let mut rect_settings = UiBuilder::new();
                rect_settings.disabled = self.container_options.container != Container::Rect;
                ui.scope_builder(rect_settings, |ui| {
                    ui.label("Viewport padding");
                    ui.horizontal(|ui| {
                        const MAX_PADDING: f32 = 45.;
                        let mut padding_perc_x =
                            camera.padding.x / camera.screen_viewport.width() * 100.;
                        let mut padding_perc_y =
                            camera.padding.y / camera.screen_viewport.height() * 100.;
                        ui.label("x : ");
                        ui.add(
                            DragValue::new(&mut padding_perc_x)
                                .speed(0.1)
                                .range(0f32..=MAX_PADDING)
                                .suffix(" %"),
                        );
                        camera.padding.x = padding_perc_x / 100. * camera.screen_viewport.width();
                        ui.label("y : ");
                        ui.add(
                            DragValue::new(&mut padding_perc_y)
                                .speed(0.1)
                                .range(0f32..=MAX_PADDING)
                                .suffix(" %"),
                        );
                        camera.padding.y = padding_perc_y / 100. * camera.screen_viewport.height();
                    });
                });
            });
            ui.checkbox(&mut self.ball_collisions, "Ball collisions");
            ui.horizontal(|ui| {
                ui.label("Solver substeps :");
                ui.add(
                    DragValue::new(&mut self.solver_iterations)
                        .speed(0.1)
                        .range(1..=10),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Energy conservation ratio : ");
                ui.add(
                    DragValue::new(&mut self.energy_conservation_perc)
                        .suffix("%")
                        .speed(0.1)
                        .range(0..=100),
                );
            });
        });
    }
}
