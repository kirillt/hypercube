//Hyperspherical coordinates

use core::*;

pub struct Polar {
    pub ro: CoordFloat,
    pub alpha: CoordFloat,
    pub beta: CoordFloat,
    pub gamma: CoordFloat
}

pub struct Angles {
    pub alpha: CoordFloat,
    pub beta: CoordFloat,
    pub gamma: CoordFloat
}

impl Polar {
    pub fn angles(&self) -> Angles {
        Angles {
            alpha: self.alpha,
            beta: self.beta,
            gamma: self.gamma
        }
    }

    pub fn from_descartes(vector: Vector) -> Self {
        let vector: &[CoordFloat] = vector.as_slice();
        let w = vector[0];
        let x = vector[1];
        let y = vector[2];
        let z = vector[3];

        let z_w = z * z + w * w;
        let y_z_w = y * y + z_w;
        let x_y_z_w = x * x + y_z_w;

        let ro = x_y_z_w.sqrt();

        let alpha = (x / x_y_z_w.sqrt()).acos();
        let beta = (y / y_z_w.sqrt()).acos();

        let gamma = (z / z_w.sqrt()).acos();
        let gamma = if w < 0. {
            2. * PI - gamma
        } else {
            gamma
        };

        Polar {
            ro, alpha, beta, gamma
        }
    }

    pub fn to_descartes(&self) -> Vector {
        let w = self.ro * self.gamma.cos();
        let x = self.ro * self.alpha.sin() * self.beta.sin() * self.gamma.sin();
        let y = self.ro * self.alpha.cos() * self.beta.sin() * self.gamma.sin();
        let z = self.ro * self.beta.cos() * self.gamma.sin();

        Vector::new(w, x, y, z)
    }
}