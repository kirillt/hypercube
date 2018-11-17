use render::Renderable;
use super::Animated;
use model;

use std::marker::PhantomData;
use std::rc::Rc;

pub struct Composition<P, Q, A, B>
where P: Renderable, Q: Renderable,
      A: Animated<P>, B: Animated<Q> {
    phantom_a: PhantomData<P>,
    phantom_b: PhantomData<Q>,

    pub first: A,
    pub second: B
}

impl<P,Q,A,B> Composition<P,Q,A,B>
where P: Renderable, Q: Renderable,
      A: Animated<P>, B: Animated<Q> {

    pub fn new(first: A, second: B) -> Self {
        Composition {
            phantom_a: PhantomData,
            phantom_b: PhantomData,

            first,
            second
        }
    }
}

pub fn compose<P: Renderable, Q: Renderable,
               A: Animated<P>, B: Animated<Q>>
        (a: A, b: B) -> Composition<P,Q,A,B> {
    Composition::new(a, b)
}

impl<P, Q, A, B> Animated<model::Composition<P,Q>> for Composition<P,Q,A,B>
where P: Renderable, Q: Renderable,
      A: Animated<P>, B: Animated<Q> {

    fn calculate(&self, time: usize) -> Rc<model::Composition<P,Q>> {
        Rc::new(model::Composition {
            first: self.first.calculate(time),
            second: self.second.calculate(time)
        })
    }

}