use eframe::egui::Context;
use info_pane::InfoPaneRenderer;
use settings_pane::SettingsPaneRenderer;

use crate::Mathsim;

mod info_pane;
mod settings_pane;

pub(crate) struct InterfaceRenderer;

impl InterfaceRenderer {
    pub(crate) fn render(app: &mut Mathsim, ctx: &Context) {
        SettingsPaneRenderer::render(app, ctx);
        InfoPaneRenderer::render(app, ctx);
    }
}
