use signifix::metric;

pub(crate) fn add_suffix(n: f32, unit: &str) -> Option<String> {
    if n == 0. {
        return Some(format!("Null ({unit})"));
    }
    let result = metric::Signifix::try_from(n).ok()?;
    Some(format!("{result}{unit}"))
}

/// Takes in a frame delay, or a frame rate value, and returns the associated frame rate value, or
/// frame delay.
pub(crate) fn delay_fps_convert(dt_or_fps: f32) -> f32 {
    1. / dt_or_fps
}
