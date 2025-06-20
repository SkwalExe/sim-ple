use eframe::egui::{Color32, Pos2, Rect, Stroke, Ui, Vec2 as EVec2};
use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::physics::{BoundingBox, Engine, engine::simulation_parameters::collisions::Container};

use super::utils::draw_polygon;

#[derive(Serialize, Deserialize)]
pub(crate) struct Camera {
    /// The screen coordinate of the world coordinate system origin.
    origin: Pos2,
    /// This should never be null, as it is initialized at the start of the first frame.
    pub(crate) screen_viewport: Rect,
    /// The viewport width in meters
    pub(crate) viewport_width: f32,
    /// The scale factor from world to screen coordinates.
    pub(crate) scale_to_screen: f32,
    /// The viewport of the camera in screen coordinates.
    /// Computed depending on screen_viewport and viewport_width.
    /// Must be updated when one of the two above is updated.
    pub(crate) viewport_world: BoundingBox,
    /// The padding between the screen viewport and the world viewport.
    pub(crate) padding: EVec2,
    /// The size of the vector arrows.
    pub(crate) arrow_size: f32,
    /// Ns: pts/N
    /// N = pts/Ns
    /// pts = Ns N
    pub(crate) newton_scale: f32,
    /// Vs: pts/m/s
    pub(crate) velocity_scale: f32,
    pub(crate) display_last_pos: bool,
    pub(crate) display_acceleration: bool,
    pub(crate) display_forces: bool,
    pub(crate) display_velocity: bool,
    pub(crate) velocity_color: Color32,
    pub(crate) forces_color: Color32,
    pub(crate) acceleration_color: Color32,
    pub(crate) last_pos_color: Color32,
}

const DEFAULT_VIEWPORT_WIDTH: f32 = 20.;
const DEFAULT_ARROW_SIZE: f32 = 5.;
const DEFAULT_CAMERA_PADDING: EVec2 = EVec2::ZERO;
const DEFAULT_NEWTON_SCALE: f32 = 0.1;
const DEFAULT_VELOCITY_SCALE: f32 = 1.;
const DEFAULT_DISPLAY_LAST_POS: bool = false;
const DEFAULT_DISPLAY_ACCELERATION: bool = false;
const DEFAULT_DISPLAY_VELOCITY: bool = false;
const DEFAULT_DISPLAY_FORCES: bool = false;
const DEFAULT_FORCES_COLOR: Color32 = Color32::from_rgb(200, 20, 200);
const DEFAULT_LAST_POS_COLOR: Color32 = Color32::RED;
const DEFAULT_ACCELERATION_COLOR: Color32 = Color32::DARK_GREEN;
const DEFAULT_VELOCITY_COLOR: Color32 = Color32::DARK_BLUE;

impl Default for Camera {
    fn default() -> Self {
        Self {
            // The origin is set to the middle of the screen on every frame.
            origin: Default::default(),
            // The screen viewport is set at the beginning of every frame.
            screen_viewport: Rect::ZERO,
            // This is stored for optimization purpose, and it is updated when necessary.
            scale_to_screen: 0.,
            // This is computed at every frame.
            viewport_world: Default::default(),
            arrow_size: DEFAULT_ARROW_SIZE,
            padding: DEFAULT_CAMERA_PADDING,
            viewport_width: DEFAULT_VIEWPORT_WIDTH,
            newton_scale: DEFAULT_NEWTON_SCALE,
            velocity_scale: DEFAULT_VELOCITY_SCALE,
            display_last_pos: DEFAULT_DISPLAY_LAST_POS,
            display_forces: DEFAULT_DISPLAY_FORCES,
            display_acceleration: DEFAULT_DISPLAY_ACCELERATION,
            display_velocity: DEFAULT_DISPLAY_VELOCITY,
            acceleration_color: DEFAULT_ACCELERATION_COLOR,
            velocity_color: DEFAULT_VELOCITY_COLOR,
            forces_color: DEFAULT_FORCES_COLOR,
            last_pos_color: DEFAULT_LAST_POS_COLOR,
        }
    }
}

impl Camera {
    pub(crate) fn render(&self, engine: &Engine, ui: &mut Ui) {
        match engine.options.collisions.container_options.container {
            Container::None => {}
            Container::Rect => {
                // Draw the padding.
                let rect = self.screen_viewport.shrink2(self.padding);
                let points = vec![
                    rect.min,
                    Pos2::new(rect.max.x, rect.min.y),
                    rect.max,
                    Pos2::new(rect.min.x, rect.max.y),
                ];
                let painter = ui.painter();
                draw_polygon(&points, Stroke::new(1., Color32::RED), painter);
            }
            Container::Circle => todo!(),
        }
    }

    pub(crate) fn viewport_width_updated(&mut self) {
        self.scale_to_screen = self.screen_viewport.max.x / self.viewport_width;
        self.origin = Pos2::new(
            (self.screen_viewport.min.x + self.screen_viewport.max.x) / 2.,
            (self.screen_viewport.min.y + self.screen_viewport.max.y) / 2.,
        );
        self.update_world_viewport();
    }
    /// Computes the viewport of the camera in world coordinates.
    pub(crate) fn update_world_viewport(&mut self) {
        let min = self.screen_viewport.min;
        let max = self.screen_viewport.max;

        // We must convert because, in screen coordinate the min point
        // is the top left corner, and the max point in the bottom right corner,
        // but in world coordinates, the min point is the bottom left,
        // and the max is the top right.
        self.viewport_world = BoundingBox {
            min: self.screen_to_world(Pos2::new(min.x + self.padding.x, max.y - self.padding.y)),
            max: self.screen_to_world(Pos2::new(max.x - self.padding.x, min.y + self.padding.y)),
        };
    }
    /// Converts a distance from world coordinates to screen coordinates
    pub(crate) fn _len_to_world(&self, len: f32) -> f32 {
        len / self.scale_to_screen
    }

    /// Converts a distance from screen coordinates to world coordinates
    pub(crate) fn len_to_screen(&self, len: f32) -> f32 {
        len * self.scale_to_screen
    }
    /// Converts screen coordinates to world coordinates.
    pub(crate) fn screen_to_world(&self, screen_pos: Pos2) -> Vec2 {
        Vec2::new(
            screen_pos.x / self.scale_to_screen - self.origin.x / self.scale_to_screen,
            self.origin.y / self.scale_to_screen - screen_pos.y / self.scale_to_screen,
        )
    }
    /// Converts world coordinates to screen coordinates.
    pub(crate) fn world_to_screen(&self, world_pos: Vec2) -> Pos2 {
        Pos2::new(
            self.origin.x + world_pos.x * self.scale_to_screen,
            self.origin.y - world_pos.y * self.scale_to_screen,
        )
    }
}
