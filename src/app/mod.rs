use std::{thread::sleep, time::Duration};

use disk_persist::DiskPersist;
use eframe::{App, CreationContext, egui::Visuals};
use log::info;
mod app_def;
mod events;
mod methods;
pub use app_def::Mathsim;

use crate::rendering::{canvas_renderer::CanvasRenderer, interface_renderer::InterfaceRenderer};

impl App for Mathsim {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // We need to ask for a repaint to update continuously.
        ctx.request_repaint();

        // Set light or dark theme, must be set on each frame
        ctx.set_visuals(if self.light_theme {
            Visuals::light()
        } else {
            Visuals::dark()
        });

        self.setup_frame(ctx);
        CanvasRenderer::render(self, ctx);
        InterfaceRenderer::render(self, ctx);

        self.engine.update(self.dt);

        // Sleep to respect the fps limit, if set.
        if self.engine.options.time.limit_fps {
            // Optimal frame duration, to get the fps limit. We could sleep this duration but it
            // wouldn't take into account the time it takes to render the frames
            let wanted_frame_delay = 1. / self.engine.options.time.max_fps as f32;

            // Only taking into account the frame render time isn't enough, too much factors are
            // involved. To get exactly to the fps limit, we must dynamically change the frame delay to
            // get closer to the limit.

            self.frame_delay += 0.1 * (wanted_frame_delay - self.dt);
            sleep(Duration::from_secs_f32(self.frame_delay.max(0.)));
        }
    }
}

impl Mathsim {
    pub fn new(_cc: &CreationContext) -> Self {
        info!("App instance created!");
        // Try to get a saved state
        let persist: DiskPersist<Self> = DiskPersist::init("mathsim").unwrap();
        persist.read().unwrap_or_default().unwrap_or_default()
    }
}
