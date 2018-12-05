use nalgebra::{Point3, Point4, Vector4, Matrix, MatrixArray, U4};
use std::rc::Rc;

pub use std::f32::consts::PI;

pub type CoordFloat = f32;
pub type Vector = Vector4<CoordFloat>;
pub type Point = Point4<CoordFloat>;

pub fn origin() -> Point { Point::origin() }
pub fn unit_xyz() -> Point { Point::new(0., 1., 1., 1.) }
pub fn unit_w() -> Point { Point::new(1., 0., 0., 0.) }
pub fn unit_x() -> Point { Point::new(0., 1., 0., 0.) }
pub fn unit_y() -> Point { Point::new(0., 0., 1., 0.) }
pub fn unit_z() -> Point { Point::new(0., 0., 0., 1.) }
pub fn unit() -> Point { Point::new(1., 1., 1., 1.) }

pub type ColorFloat = f32;
pub type Color = Point3<ColorFloat>;

pub fn black() -> Color { Color::new(0., 0., 0.) }
pub fn red() -> Color { Color::new(1., 0., 0.) }
pub fn yellow() -> Color { Color::new(1., 1., 0.) }
pub fn green() -> Color { Color::new(0., 1., 0.) }
pub fn cyan() -> Color { Color::new(0., 1., 1.) }
pub fn blue() -> Color { Color::new(0., 0., 1.) }
pub fn purple() -> Color { Color::new(1., 0., 1.) }
pub fn white() -> Color { Color::new(1., 1., 1.) }

pub fn mix(a: Color, b: Color, grade: ColorFloat) -> Color {
    Color::from(a.coords * (1. - grade) + b.coords * grade)
}

pub type Matrix4 = Matrix<CoordFloat, U4, U4,
    MatrixArray<CoordFloat, U4, U4>>;

pub fn scale(factor: CoordFloat) -> Matrix4 {
    Matrix4::new(factor, 0.,     0.,     0.,
                 0.,     factor, 0.,     0.,
                 0.,     0.,     factor, 0.,
                 0.,     0.,     0.,     factor)
}

pub type Refs<T> = Vec<Rc<T>>;

pub fn clone_points(refs: Refs<Vec<Point>>) -> Vec<Point> {
    let mut points = vec![];
    for chunk in refs.into_iter() {
        for point in chunk.iter() {
            points.push(point.clone())
        }
    }
    points
}

pub fn clone_colors(refs: Refs<Vec<Color>>) -> Vec<Color> {
    let mut colors = vec![];
    for chunk in refs.into_iter() {
        for color in chunk.iter() {
            colors.push(color.clone())
        }
    }
    colors
}

pub fn flatten_coordinates(refs: Refs<Vec<Point>>) -> Vec<CoordFloat> {
    let mut coordinates_flat = vec![];
    for chunk in refs.into_iter() {
        for point in chunk.iter() {
            for coord in point.iter().skip(1) {
                coordinates_flat.push(*coord);
            }
        }
    }
    coordinates_flat
}

pub fn flatten_components(refs: Refs<Vec<Color>>) -> Vec<ColorFloat> {
    let mut components_flat = vec![];
    for chunk in refs.into_iter() {
        for color in chunk.iter() {
            for comp in color.iter() {
                components_flat.push(*comp);
            }
        }
    }
    components_flat
}

pub fn push_square(indices: &mut Vec<u16>, a: u16, b: u16, c: u16, d: u16) {
    push_triangle(indices, a, b, c);
    push_triangle(indices, b, c, d);
}

pub fn push_triangle(indices: &mut Vec<u16>, a: u16, b: u16, c: u16) {
    indices.push(a);
    indices.push(b);
    indices.push(c);
}

pub fn js_assert(condition: bool, strict: bool, message: String) {
    if !condition {
        js_error(message);
        if strict {
            panic!("Assertion failed");
        }
    }
}

pub fn js_error(message: String) {
    js! { console.error(@{message}); }
}

pub fn js_log(message: String) {
    js! { console.log(@{message}); }
}