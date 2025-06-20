use eframe::egui::Ui;

use crate::{physics::Engine, rendering::PointsRenderer};

use super::Camera;

impl Engine {
    pub(crate) fn render(&self, camera: &Camera, ui: &mut Ui) {
        PointsRenderer::render(camera, ui, self);
    }
}
