use vector::*;

use std::iter::Extend;

#[derive(Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b}
    }

    pub fn from_vector(v: &Vector) -> Color { //hack
        Color {
            r: v.x,
            g: v.y,
            b: v.z
        }
    }
}

pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0 };
pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0 };
pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0 };

pub const GRAY: Color = Color { r: 0.5, g: 0.5, b: 0.5 };

#[derive(Clone)]
pub struct Point {
    pub position: Vector,
    pub color: Color
}

#[derive(Clone)]
pub struct Simplex2D {
    pub a: Point,
    pub b: Point,
    pub c: Point
}

pub trait Renderable {

    fn render(self) -> Vec<Simplex2D>;

}

impl Simplex2D {

    pub fn encode(self) -> Vec<f32> {
        vec![self.a.position.x, self.a.position.y, self.a.position.z,
             self.a.color.r, self.a.color.g, self.a.color.b,

             self.b.position.x, self.b.position.y, self.b.position.z,
             self.b.color.r, self.b.color.g, self.b.color.b,

             self.c.position.x, self.c.position.y, self.c.position.z,
             self.c.color.r, self.c.color.g, self.c.color.b
        ]
    }

    pub fn encode_all(simplexes: Vec<Simplex2D>) -> Vec<f32> {
        let mut result = vec![];
        for simplex in simplexes {
            result.extend(simplex.encode());
        }
        result
    }

}


impl Shiftable for Point {
    fn shift(&self, v: &Vector) -> Point {
        Point {
            position: self.position.shift(v),
            color: self.color.clone()
        }
    }
}

impl Rotatable for Point {
    fn rotate(&self, v: &Vector) -> Point {
        Point {
            position: self.position.rotate(v),
            color: self.color.clone()
        }
    }
}

impl Scalable for Point {
    fn scale(&self, v: &Vector) -> Point {
        Point {
            position: self.position.scale(v),
            color: self.color.clone()
        }
    }
}