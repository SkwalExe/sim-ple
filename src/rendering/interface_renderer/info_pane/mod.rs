use std::fmt::Display;

use eframe::egui::{Align2, Context, TextFormat, Ui, Window, text::LayoutJob};
use egui::ScrollArea;

use crate::{Mathsim, utils::add_suffix};

fn info_line(ui: &mut Ui, label: impl Display, value: impl Display) {
    let mut text = LayoutJob::default();
    text.append(&format!("{label} : "), 0., TextFormat::default());
    text.append(
        &format!(" {} ", value),
        0.,
        TextFormat {
            background: ui.visuals().extreme_bg_color,
            ..Default::default()
        },
    );
    ui.label(text);
}

pub(crate) struct InfoPaneRenderer;

impl InfoPaneRenderer {
    pub(crate) fn render(app: &mut Mathsim, ctx: &Context) {
        // We have to use this little trick because .open would borrow app.
        let mut info_pane_open = app.info_pane_open;
        Window::new("Info")
            .open(&mut info_pane_open)
            .anchor(Align2::RIGHT_TOP, [-20., 20.])
            .show(ctx, |ui| {
                // ======== Ball count
                info_line(ui, "Ball count", app.engine.points.len());
                // ======== Kinetic energy
                let kinetic_energy = app.engine.total_kinetic_energy();
                let kinetic_energy_formatted =
                    add_suffix(kinetic_energy, "J").unwrap_or("Could not compute!!".to_string());
                info_line(ui, "Total kinetic energy", kinetic_energy_formatted);
                // ======== Potential energy
                let potential_energy = app.engine.total_potential_energy();
                let potential_energy_formatted =
                    add_suffix(potential_energy, "J").unwrap_or("Could not compute!!".to_string());
                info_line(ui, "Total potential energy", potential_energy_formatted);
                // ======== Mecanic energy
                let mecanic_energy = kinetic_energy + potential_energy;
                let mecanic_energy_formatted =
                    add_suffix(mecanic_energy, "J").unwrap_or("Could not compute!!".to_string());
                info_line(ui, "Total mecanic energy", mecanic_energy_formatted);
                // ======== FPS
                info_line(
                    ui,
                    "Average frame duration",
                    format!("{:.2} ms", app.avg_dt() * 1000.),
                );
                info_line(
                    ui,
                    "Average frame rate",
                    format!("{:.2} fps", app.get_fps()),
                );
                ui.separator();
                ui.label("Hover to locate point. Right click to delete.");
                ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        // Remove points where the button is right clicked
                        app.engine.points.retain_mut(|p| {
                            let button = ui.button(format!("Point {}", p.id));
                            p.tracked = button.hovered();
                            !button.clicked_by(egui::PointerButton::Secondary)
                        });
                    })
                })
            });
        app.info_pane_open = info_pane_open;
    }
}
