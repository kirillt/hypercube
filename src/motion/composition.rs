use core::*;
use motion::Animated;

pub struct Composition<A, B>
where A: Animated, B: Animated {
    pub first: A,
    pub second: B
}

impl<A, B> Animated for Composition<A,B>
where A: Animated, B: Animated {

    fn positions(&self, time: usize) -> Refs<Vec<Point>> {
        let mut result = self.first.positions(time);
        let mut other = self.second.positions(time);
        result.append(&mut other);
        result
    }

    fn colors(&self, time: usize) -> Refs<Vec<Color>> {
        let mut result = self.first.colors(time);
        let mut other = self.second.colors(time);
        result.append(&mut other);
        result
    }

    fn indices(&self) -> Vec<u16> {
        let k = self.first.size();
        let mut result = self.first.indices();

        let mut second: Vec<u16> =
            self.second.indices().into_iter()
                .map(move |i| i + k)
                .collect();

        result.append(&mut second);
        result
    }

    fn size(&self) -> u16 {
        self.first.size() + self.second.size()
    }
}

impl<A, B> Composition<A,B>
where A: Animated, B: Animated {

    fn new(a: A, b: B) -> Self {
        Composition { first: a, second: b }
    }

}

pub fn compose<A: Animated, B: Animated>(a: A, b: B) -> Composition<A,B> {
    Composition::new(a, b)
}