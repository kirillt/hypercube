#[derive(Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl<'a> Into<Vec<f32>> for &'a Vector {
    fn into(self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }
}

pub const ORIGIN: Vector = Vector { x: 0.0, y: 0.0, z: 0.0 };

pub const INVERT: Vector = Vector { x: -1.0, y: -1.0, z: -1.0};

pub const UNIT_X: Vector = Vector { x: 1.0, y: 0.0, z: 0.0 };
pub const UNIT_Y: Vector = Vector { x: 0.0, y: 1.0, z: 0.0 };
pub const UNIT_Z: Vector = Vector { x: 0.0, y: 0.0, z: 1.0 };

pub const UNIT: Vector = Vector { x: 1.0, y: 1.0, z: 1.0 };

impl Vector {
    pub fn with_x(&self, c: f32) -> Vector {
        Vector { x: c, y: self.y, z: self.z }
    }

    pub fn with_y(&self, c: f32) -> Vector {
        Vector { x: self.x, y: c, z: self.z }
    }

    pub fn with_z(&self, c: f32) -> Vector {
        Vector { x: self.x, y: self.y, z: c }
    }


    pub fn multiply(&self, v: &Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.x * v.z - self.z * v.x,
            z: self.x * v.y - self.y * v.x
        }
    }
}

pub trait Shiftable: Sized {

    fn shift(&self, v: &Vector) -> Self {
        self.shift_x(v.x)
            .shift_y(v.y)
            .shift_z(v.z)
    }

    fn shift_back(&self, v: &Vector) -> Self {
        self.shift_(v.mirror())
    }

    fn shift_x(&self, r: f32) -> Self {
        self.shift_(ORIGIN.with_x(r))
    }

    fn shift_y(&self, r: f32) -> Self {
        self.shift_(ORIGIN.with_y(r))
    }

    fn shift_z(&self, r: f32) -> Self {
        self.shift_(ORIGIN.with_z(r))
    }

    fn shift_back_(&self, v: Vector) -> Self {
        self.shift_back(&v)
    }

    fn shift_(&self, v: Vector) -> Self {
        self.shift(&v)
    }

}

pub trait Rotatable: Sized {

    fn rotate(&self, v: &Vector) -> Self {
        self.rotate_xy(v.x)
            .rotate_yz(v.y)
            .rotate_zx(v.z)
    }

    fn rotate_xy(&self, a: f32) -> Self {
        self.rotate_(ORIGIN.with_x(a))
    }

    fn rotate_yz(&self, a: f32) -> Self {
        self.rotate_(ORIGIN.with_y(a))
    }

    fn rotate_zx(&self, a: f32) -> Self {
        self.rotate_(ORIGIN.with_z(a))
    }

    fn rotate_(&self, v: Vector) -> Self {
        self.rotate(&v)
    }

}

pub trait Scalable: Sized {

    fn scale(&self, v: &Vector) -> Self {
        self.scale_x(v.x)
            .scale_y(v.y)
            .scale_z(v.z)
    }

    fn scale_eq(&self, q: f32) -> Self {
        self.scale_(Vector { x: q, y: q, z: q})
    }

    fn scale_x(&self, q: f32) -> Self {
        self.scale_(UNIT.with_x(q))
    }

    fn scale_y(&self, q: f32) -> Self {
        self.scale_(UNIT.with_y(q))
    }

    fn scale_z(&self, q: f32) -> Self {
        self.scale_(UNIT.with_z(q))
    }

    fn scale_(&self, v: Vector) -> Self {
        self.scale(&v)
    }


    fn mirror_x(&self) -> Self {
        self.scale_x(-1.0)
    }

    fn mirror_y(&self) -> Self {
        self.scale_y(-1.0)
    }

    fn mirror_z(&self) -> Self {
        self.scale_z(-1.0)
    }

    fn mirror(&self) -> Self {
        self.scale_(INVERT)
    }
}

impl Shiftable for Vector {
    fn shift(&self, v: &Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    fn shift_x(&self, r: f32) -> Vector {
        self.with_x(self.x + r)
    }

    fn shift_y(&self, r: f32) -> Vector {
        self.with_y(self.y + r)
    }

    fn shift_z(&self, r: f32) -> Vector {
        self.with_z(self.z + r)
    }
}

impl Rotatable for Vector {
    fn rotate_xy(&self, a: f32) -> Vector {
        Vector {
            x: self.x * a.cos() - self.y * a.sin(),
            y: self.y * a.cos() + self.x * a.sin(),
            z: self.z
        }
    }

    fn rotate_yz(&self, a: f32) -> Vector {
        Vector {
            x: self.x,
            y: self.y * a.cos() - self.z * a.sin(),
            z: self.z * a.cos() + self.y * a.sin()
        }
    }

    fn rotate_zx(&self, a: f32) -> Vector {
        Vector {
            x: self.x * a.cos() + self.z * a.sin(),
            y: self.y,
            z: self.z * a.cos() - self.x * a.sin()
        }
    }
}

impl Scalable for Vector {
    fn scale(&self, v: &Vector) -> Vector {
        Vector {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }

    fn scale_x(&self, q: f32) -> Vector {
        self.with_x(self.x * q)
    }

    fn scale_y(&self, q: f32) -> Vector {
        self.with_y(self.y * q)
    }

    fn scale_z(&self, q: f32) -> Vector {
        self.with_z(self.z * q)
    }
}