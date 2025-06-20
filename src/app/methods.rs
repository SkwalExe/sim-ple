use eframe::egui::Context;

use crate::utils::delay_fps_convert;

use super::Mathsim;

impl Mathsim {
    /// Increment the frame counter, compute and record dt, log performence.
    pub(crate) fn setup_frame(&mut self, ctx: &Context) {
        // We use continuous mode so using unstable_dt should never cause issues.
        self.dt = ctx.input(|i| i.unstable_dt);
        self.record_dt(self.dt);

        self.frame_counter += 1;

        self.camera.screen_viewport = ctx.screen_rect();
        self.camera.viewport_width_updated();
        self.engine.viewport = self.camera.viewport_world.clone();
    }
    fn record_dt(&mut self, dt: f32) {
        self.last_dts.push(dt);
        const DT_HISTORY_LEN: usize = 100;
        if self.last_dts.len() > DT_HISTORY_LEN {
            let _ = self.last_dts.remove(0);
        }
    }

    pub(crate) fn avg_dt(&self) -> f32 {
        self.last_dts.iter().sum::<f32>() / self.last_dts.len() as f32
    }

    pub(crate) fn get_fps(&self) -> f32 {
        delay_fps_convert(self.avg_dt())
    }
}
