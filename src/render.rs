use vector::*;

#[derive(Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl<'a> Into<Vec<f32>> for &'a Color {
    fn into(self) -> Vec<f32> {
        vec![self.r, self.g, self.b]
    }
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

pub trait Renderable {
    fn positions(&self) -> Vec<Vector>;

    fn colors(&self) -> Vec<Color>;

    fn indices(&self) -> Vec<u16>;

    fn positions_flat(&self) -> Vec<f32> {
        self.positions().iter()
            .map(|v| { let v: Vec<f32> = v.into(); v.into_iter() })
            .flatten().collect()
    }

    fn colors_flat(&self) -> Vec<f32> {
        self.colors().iter()
            .map(|v| { let v: Vec<f32> = v.into(); v.into_iter() })
            .flatten().collect()
    }
}