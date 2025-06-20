use eframe::egui::{Color32, PointerButton, Sense, Stroke, Ui};

use crate::{Mathsim, rendering::VecRenderer};

impl Mathsim {
    pub(crate) fn handle_mouse_input(&mut self, ui: &mut Ui) {
        let response = ui.allocate_rect(ui.max_rect(), Sense::click_and_drag());
        // Try to get the position of the pointer. It it is None, then
        // no mouse input was detected.
        let Some(pos) = response.interact_pointer_pos() else {
            return;
        };

        if response.clicked()
            || (response.dragged_by(PointerButton::Secondary)
                && !response.dragged_by(PointerButton::Primary))
        {
            // On a simple click, add a point without velocity.
            self.engine
                .add_default_point(self.camera.screen_to_world(pos));
        } else if response.drag_started_by(PointerButton::Primary) {
            // On drag start, save the start position
            self.drag_started = pos;
        } else if response.drag_stopped_by(PointerButton::Primary)
            || response.dragged_by(PointerButton::Secondary)
        {
            // On drag end, add a new point with velocity
            let displacement =
                self.camera.screen_to_world(pos) - self.camera.screen_to_world(self.drag_started);
            let new_point = self
                .engine
                .options
                .new_points
                .default_point
                .clone()
                .with_pos(self.camera.screen_to_world(self.drag_started))
                .with_vel(displacement * 4.);
            self.engine.add_point(new_point);
        }

        // If we are currently dragging, draw the velocity vector.
        if response.dragged_by(PointerButton::Primary) {
            let painter = ui.painter();
            painter.extend(VecRenderer::get_shapes(
                self.drag_started,
                pos,
                Stroke::new(2., Color32::RED),
                self.camera.arrow_size,
            ));
        }
    }
}
