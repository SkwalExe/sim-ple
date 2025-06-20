use eframe::egui::Ui;

use super::Mathsim;
mod keyboard_handling;
mod mouse_handling;

impl Mathsim {
    /// The input is handled before the simulation.
    pub(crate) fn handle_input(&mut self, ui: &mut Ui) {
        self.handle_mouse_input(ui);
        self.handle_keyboard_input(ui);
    }
}
