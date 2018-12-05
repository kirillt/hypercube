use core::*;
use model::transform::*;
use motion::animated::Animated;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Rotation<A: Animated> {
    pub angles: Vec<CoordFloat>,
    pub threshold_ms: usize,
    last_rendered: RefCell<usize>,
    positions: RefCell<Rc<Vec<Point>>>,
    object: A,
}

impl<A: Animated> Animated for Rotation<A> {
    fn positions(&self, time: usize) -> Refs<Vec<Point>> {
        let elapsed = {
            time - *self.last_rendered.borrow()
        };

        let result = if elapsed < self.threshold_ms {
            self.positions.borrow().clone()
        } else {
            let angles = self.angles.iter()
                .map(|a| a * (time as CoordFloat / 1000.))
                .collect();

            let transformation = rotation_matrix(angles);
            //possible to optimize with usage of state

            let points: Vec<Point> =
                clone_points(self.object.positions(time))
                .into_iter()
                .map(|p| transformation * p)
                .collect();

            self.last_rendered.replace(time);

            let points: Rc<Vec<Point>> = Rc::new(points);
            self.positions.replace(points.clone());
            points
        };

        vec![result]
    }

    fn colors(&self, time: usize) -> Refs<Vec<Color>> {
        self.object.colors(time)
    }

    fn indices(&self) -> Vec<u16> {
        self.object.indices()
    }

    fn size(&self) -> u16 {
        self.object.size()
    }
}

impl<A: Animated> Rotation<A> {
    pub fn d3(object: A, mut angles: Vec<CoordFloat>, fps_limit: usize) -> Self {
        assert!(angles.len() == 3);
        angles.push(0.);
        angles.push(0.);
        angles.push(0.);

        Rotation::d4(object, angles, fps_limit)
    }

    pub fn d4(object: A, angles: Vec<CoordFloat>, fps_limit: usize) -> Self {
        assert!(angles.len() == 6);
        Rotation {
            angles,
            threshold_ms: 1000 / fps_limit,
            last_rendered: RefCell::new(0),
            positions: RefCell::new(Rc::new(
                clone_points(object.positions(0))
            )),
            object
        }
    }
}