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

impl Renderable for Tetrahedron {

    fn render(self) -> Vec<Simplex2D> {
        let base = &self.base;
        let color = &self.peak.color;

        let abd = Triangle { a: base.a.clone(), b: base.b.clone(), c: self.peak.position.clone(), color: color.clone() };
        let acd = Triangle { a: base.a.clone(), b: base.c.clone(), c: self.peak.position.clone(), color: color.clone() };
        let bcd = Triangle { a: base.b.clone(), b: base.c.clone(), c: self.peak.position.clone(), color: color.clone() };

        vec![ base.clone().render(), abd.render(), acd.render(), bcd.render() ]
    }

}