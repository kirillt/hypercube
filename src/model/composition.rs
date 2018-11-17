use render::*;
use vector::Vector;

use std::rc::Rc;

pub struct Composition<A: Renderable, B: Renderable> {
    pub first: Rc<A>,
    pub second: Rc<B>
}

impl<A,B> Composition<A,B>
where A: Renderable, B: Renderable {

    pub fn new(first: A, second: B) -> Self {
        Composition {
            first: Rc::new(first),
            second: Rc::new(second)
        }
    }

}

pub fn compose<A: Renderable, B: Renderable>(a: A, b: B) -> Composition<A,B> {
    Composition::new(a, b)
}

impl<A,B> Renderable for Composition<A,B> where A: Renderable, B: Renderable {
    fn positions(&self) -> Vec<Vector> {
        let mut result = self.first.positions();
        result.append(&mut self.second.positions());
        result
    }

    fn colors(&self) -> Vec<Color> {
        let mut result = self.first.colors();
        result.append(&mut self.second.colors());
        result
    }

    fn indices(&self) -> Vec<u16> {
        let mut result = self.first.indices();
        let n = self.first.positions().len() as u16;

        let mut second: Vec<u16> =
            self.second.indices().into_iter()
                .map(move |i| i + n)
                .collect();

        result.append(&mut second);
        result
    }
}