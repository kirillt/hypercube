use render::*;
use vector::*;

use itertools::Itertools;

pub fn build<T: Renderable>(bottom: T, top: T) -> Prism<T> { //, offset: usize
    let n = bottom.positions().len();
    if n != top.positions().len() {
        panic!("Can't attach top to bottom of different amount of vertices");
    }

    let n = n as u16;

    Prism {
        n,

        bottom,
        top
    }
}

pub struct Prism<T: Renderable> {
    n: u16,
    bottom: T,
    top: T
}

impl<T> Shiftable for Prism<T> where T: Shiftable + Renderable {
    fn shift(&self, v: &Vector) -> Prism<T> {
        Prism { n: self.n, bottom: self.bottom.shift(v), top: self.top.shift(v) }
    }
}

impl<T> Rotatable for Prism<T> where T: Rotatable + Renderable {
    fn rotate(&self, v: &Vector) -> Prism<T> {
        Prism { n: self.n, bottom: self.bottom.rotate(v), top: self.top.rotate(v) }
    }
}

impl<T> Scalable for Prism<T> where T: Scalable + Renderable {
    fn scale(&self, v: &Vector) -> Prism<T> {
        Prism { n: self.n, bottom: self.bottom.scale(v), top: self.top.scale(v) }
    }
}

impl<T> Renderable for Prism<T> where T: Renderable {
    fn positions(&self) -> Vec<Vector> {
        let mut result = self.bottom.positions();
        result.append(&mut self.top.positions());
        result
    }

    fn colors(&self) -> Vec<Color> {
        let mut result = self.bottom.colors();
        result.append(&mut self.top.colors());
        result
    }

    fn indices(&self) -> Vec<u16> {
        let n = self.n;
        let mut result = self.bottom.indices();

        fn renumerate(n: u16, iter: impl Iterator<Item=u16>) -> impl Iterator<Item=u16> {
            iter.map(move |i| i + n)
        }

        let mut top: Vec<u16> =
            renumerate(n, self.top.indices().into_iter())
            .collect();

        result.append(&mut top);

        let mut edges: Vec<(u16, u16)> = (0..n)
            .zip(renumerate(n, 0..n))
            .collect();
        edges.push((0, n));

        for ((a,b),(c,d)) in edges.into_iter().tuple_windows() {
            result.push(a);
            result.push(c);
            result.push(d);

            result.push(a);
            result.push(b);
            result.push(d);
        }

        result
    }
}