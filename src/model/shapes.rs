use figures::*;
use render::*;
use vector::*;
use render;

pub struct Tetrahedron {
    pub base: Triangle,
    pub peak: Vector
}

impl Shiftable for Tetrahedron {
    fn shift(&self, v: &Vector) -> Tetrahedron {
        Tetrahedron { base: self.base.shift(v), peak: self.peak.shift(v) }
    }
}

impl Rotatable for Tetrahedron {
    fn rotate(&self, v: &Vector) -> Tetrahedron {
        Tetrahedron { base: self.base.rotate(v), peak: self.peak.rotate(v) }
    }
}

impl Scalable for Tetrahedron {
    fn scale(&self, v: &Vector) -> Tetrahedron {
        Tetrahedron { base: self.base.scale(v), peak: self.peak.scale(v) }
    }
}

impl Renderable for Tetrahedron {
    fn positions(&self) -> Vec<Vector> {
        let mut result = self.base.positions();
        result.push(self.peak.clone());
        result
    }

    fn colors(&self) -> Vec<Color> {
        let color = &self.base.color;
        vec![color.clone(), color.clone(), color.clone(), render::RED] //debug
    }

    fn indices(&self) -> Vec<u16> {
        vec![0, 1, 2, 0, 1, 3, 1, 2, 3, 2, 0, 3]
    }
}