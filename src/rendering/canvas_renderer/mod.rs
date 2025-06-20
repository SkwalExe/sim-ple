use eframe::egui::{CentralPanel, Context, Frame};

use crate::Mathsim;
pub(crate) struct CanvasRenderer;

impl CanvasRenderer {
    pub(crate) fn render(app: &mut Mathsim, ctx: &Context) {
        CentralPanel::default()
            // Set a darker background color
            .frame(Frame::default().fill(ctx.style().visuals.extreme_bg_color))
            .show(ctx, |ui| {
                app.handle_input(ui);
                app.engine.render(&app.camera, ui);
                app.camera.render(&app.engine, ui)
            });
    }
}
