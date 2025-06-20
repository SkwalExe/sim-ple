use eframe::egui::{Event, Ui};
use log::info;

use crate::Mathsim;

impl Mathsim {
    fn handle_key(&mut self, k: &String) {
        match k.as_str() {
            "r" => self.engine.points.clear(),
            _ => {}
        }
    }

    pub(crate) fn handle_keyboard_input(&mut self, ui: &mut Ui) {
        ui.ctx().input(|i| {
            for event in &i.events {
                match event {
                    Event::Text(k) => {
                        info!("Event: Key pressed {:?}", k);
                        self.handle_key(k);
                    }
                    _ => {}
                }
            }
        });
    }
}
