use vector::*;
use render::*;
use render;

#[derive(Clone)]
pub struct Triangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,

    pub color: Color
}

impl Triangle {
    pub fn opposite(&self) -> Vector {
        self.a.shift_(self.c.shift_back_(self.b.clone()))
    }
}

impl Shiftable for Triangle {
    fn shift(&self, v: &Vector) -> Triangle {
        Triangle {
            a: self.a.shift(v),
            b: self.b.shift(v),
            c: self.c.shift(v),

            color: self.color.clone()
        }
    }
}

impl Rotatable for Triangle {
    fn rotate(&self, v: &Vector) -> Triangle {
        Triangle {
            a: self.a.rotate(v),
            b: self.b.rotate(v),
            c: self.c.rotate(v),

            color: self.color.clone()
        }
    }
}

impl Scalable for Triangle {
    fn scale(&self, v: &Vector) -> Triangle {
        Triangle {
            a: self.a.scale(v),
            b: self.b.scale(v),
            c: self.c.scale(v),

            color: self.color.clone()
        }
    }
}

impl Renderable for Triangle {
    fn positions(&self) -> Vec<Vector> {
        vec![self.a.clone(), self.b.clone(), self.c.clone()]
    }

    fn colors(&self) -> Vec<Color> {
        vec![self.color.clone(), self.color.clone(), self.color.clone()]
    }

    fn indices(&self) -> Vec<u16> {
        vec![0, 1, 2]
    }
}


pub struct Diamond {
    pub half: Triangle
}

impl Shiftable for Diamond {
    fn shift(&self, v: &Vector) -> Diamond {
        Diamond { half: self.half.shift(v) }
    }
}

impl Rotatable for Diamond {
    fn rotate(&self, v: &Vector) -> Diamond {
        Diamond { half: self.half.rotate(v) }
    }
}

impl Scalable for Diamond {
    fn scale(&self, v: &Vector) -> Diamond {
        Diamond { half: self.half.scale(v) }
    }
}

impl Renderable for Diamond {
    fn positions(&self) -> Vec<Vector> {
        let mut result = self.half.positions();
        result.push(self.half.opposite());
        result
    }

    fn colors(&self) -> Vec<Color> {
        let color = &self.half.color;
        vec![color.clone(), color.clone(), color.clone(), render::RED] //debug
    }

    fn indices(&self) -> Vec<u16> {
        vec![0, 1, 2, 0, 2, 3]
    }
}