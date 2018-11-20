use nalgebra::{Point3, Vector3, Matrix, MatrixArray, U3};
use std::rc::Rc;

pub type CoordFloat = f32;
pub type Vector = Vector3<CoordFloat>;
pub type Point = Point3<CoordFloat>;

pub fn unit_x() -> Point { Point::new(1., 0., 0.) }
pub fn unit_y() -> Point { Point::new(0., 1., 0.) }
pub fn unit_z() -> Point { Point::new(0., 0., 1.) }
pub fn unit() -> Point { Point::new(1., 1., 1.) }

pub type ColorFloat = f32;
pub type Color = Point3<ColorFloat>;

pub fn red() -> Color { Color::new(1., 0., 0.) }
pub fn green() -> Color { Color::new(0., 1., 0.) }
pub fn blue() -> Color { Color::new(0., 0., 1.) }

pub type Matrix3 = Matrix<CoordFloat, U3, U3,
    MatrixArray<CoordFloat, U3, U3>>;

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
            for coord in point.iter() {
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