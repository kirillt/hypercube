use nalgebra::{Point3, Vector3};
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

pub type Refs<T> = Vec<Rc<T>>;