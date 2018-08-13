use render::*;
use vector::Vector;

pub struct Composition<A: Renderable, B: Renderable> {
    pub first: A,
    pub second: B
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