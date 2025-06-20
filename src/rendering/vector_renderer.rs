use eframe::egui::{Pos2, Shape, Stroke, Vec2 as EVec2};

pub(crate) struct VecRenderer;

impl VecRenderer {
    pub(crate) fn get_shapes(a: Pos2, b: Pos2, stroke: Stroke, arrow_size: f32) -> Vec<Shape> {
        //
        //                          d
        //                          | \
        //                          |   \
        // a------------------------c---b
        //                          |  /
        //                          | /
        //                          e
        //
        //
        let mut result = Vec::with_capacity(3);
        result.push(Shape::line_segment([a, b], stroke));
        let ab = b - a;
        let ab_unit = ab.normalized();
        let normal_unit = EVec2::new(-ab_unit.y, ab_unit.x);

        let c = b - ab_unit * arrow_size;
        let d = c + normal_unit * arrow_size;
        let e = c - normal_unit * arrow_size;

        result.push(Shape::line_segment([b, d], stroke));
        result.push(Shape::line_segment([b, e], stroke));
        result
    }
}
