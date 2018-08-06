use figures::*;
use render::*;
use vector::*;

pub struct Tetrahedron {
    pub base: Triangle,
    pub peak: Point
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

//impl Renderable for Tetrahedron {
//    fn positions(self) -> Vec<Vector> {
//        vec![self.a, self.b, self.c]
//    }
//
//    fn colors(self) -> Vec<Color> {
//        vec![self.color, self.color, self.color]
//    }
//
//    fn indices(self) -> Vec<u16> {
//        vec![0, 1, 2]
//    }
//}