use vector::*;
use render::*;

#[derive(Clone)]
pub struct Triangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,

    pub color: Color
}

pub struct Diamond {
    pub half: Triangle
}

impl Triangle {
    pub fn neighbour(&self) -> Triangle {
        Triangle {
            a: self.a.clone(),
            b: self.a.shift_(self.c.shift_back_(self.b.clone())),
            c: self.c.clone(),

            color: self.color.clone()
        }
    }

    pub fn render(self) -> Simplex2D {
        let brrr = self.clone();
        Simplex2D {
            a: Point { position: self.a, color: self.color.clone() }, //color: Color::from_vector(&brrr.a) }, //
            b: Point { position: self.b, color: self.color.clone() }, //color: Color::from_vector(&brrr.b) }, //
            c: Point { position: self.c, color: self.color.clone() }, //color: Color::from_vector(&brrr.c) }, //
        }
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
    fn render(self) -> Vec<Simplex2D> {
        vec![self.render()]
    }
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
    fn render(self) -> Vec<Simplex2D> {
        vec![
            self.half.clone().render(),
            self.half.neighbour().render()
        ]
    }
}

