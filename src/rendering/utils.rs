use eframe::egui::{Painter, Pos2, Shape, Stroke};

/// Draw a polygon with the given painter.
pub(crate) fn draw_polygon(points: &Vec<Pos2>, stroke: Stroke, painter: &Painter) {
    let len = points.len();
    let mut shapes = Vec::with_capacity(len);
    for i in 0..len {
        let point = points[i];
        let next_point = points[(i + 1) % len];
        shapes.push(Shape::line_segment([point, next_point], stroke));
    }
    painter.extend(shapes);
}
