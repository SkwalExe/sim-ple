use derivative::Derivative;
use eframe::egui::Pos2;
use serde::{Deserialize, Serialize};

use crate::{physics::Engine, rendering::Camera};

/// Base structure encapsulating all the app logic.
#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub struct Mathsim {
    /// The Physics Engine settings, managing the simulation.
    pub(crate) engine: Engine,
    /// Camera settings, used for managing rendering logic.
    pub(crate) camera: Camera,
    /// Used to record the last DTs, to compute a frame rate and duration average.
    /// The DTs are stored in seconds, and the most recent one is at the end of the vector.
    #[serde(skip)]
    pub(crate) last_dts: Vec<f32>,
    /// No actual use as of today, but could have a purpose in the future.
    /// Counts the number of frame since the start of the app.
    #[serde(skip)]
    pub(crate) frame_counter: i32,
    /// The duration for which the last finished frame was displayed, in seconds.
    #[serde(skip)]
    pub(crate) dt: f32,
    /// Whether or not to use light theme for the user interface.
    pub(crate) light_theme: bool,
    /// The time, in seconds, to sleep at the end of a frame to avoid going above the FPS limit.
    /// This value is updated dynamically, if the FPS is to low or too high.
    #[serde(skip)]
    pub(crate) frame_delay: f32,
    /// During a mouse drag event, the screen position at which the mouse was first clicked.
    #[serde(skip)]
    pub(crate) drag_started: Pos2,
    /// Whether or not to render the settings pane.
    #[derivative(Default(value = "true"))]
    pub(crate) settings_pane_open: bool,
    /// Whether or not to render the info pane.
    #[derivative(Default(value = "true"))]
    pub(crate) info_pane_open: bool,
}
