use render::Renderable;

use std::rc::Rc;

pub trait Animated<R: Renderable> {
    fn calculate(&self, time: usize) -> Rc<R>;
}

pub struct Constant<R> {
    object: Rc<R>
}

impl <R: Renderable> Constant<R> {
    pub fn new(renderable: R) -> Constant<R> {
        Constant {
            object: Rc::new(renderable)
        }
    }
}

impl <R: Renderable> Animated<R> for Constant<R> {
    fn calculate(&self, _time: usize) -> Rc<R> {
        self.object.clone()
    }
}

