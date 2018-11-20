use core::*;
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
//        let debug1 = format!("{}", elapsed);
//        js! { console.log("elapsed: " + @{debug1}); }

        let result = if elapsed < self.threshold_ms {
            self.positions.borrow().clone()
        } else {
            let angles = self.angles.iter()
                .map(|a| a * (time as CoordFloat / 1000.))
                .collect();

            let debug2 = format!("{:?}", angles);
            js! { console.log("angles: " + @{debug2}); }

            let transformation = rotation_matrix(angles);

            let points =
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
    pub fn new(object: A, angles: Vec<CoordFloat>, fps_limit: usize) -> Self {
        assert!(angles.len() == 3);
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

pub fn rotation_matrix(angles: Vec<CoordFloat>) -> Matrix3 {
    assert!(angles.len() == 3);
    let angle_x = angles[0];
    let angle_y = angles[1];
    let angle_z = angles[2];

    let transformation_x = Matrix3::new(
        1.,            0.,                  0.,
        0., angle_x.cos(), -1. * angle_x.sin(),
        0., angle_x.sin(),       angle_x.cos()
    );

    let transformation_y = Matrix3::new(
        angle_y.cos(), 0., angle_y.sin(),
        0., 1.,            0.,
        -1. * angle_y.sin(), 0., angle_y.cos()
    );

    let transformation_z = Matrix3::new(
        angle_z.cos(), -1. * angle_z.sin(), 0.,
        angle_z.sin(),       angle_z.cos(), 0.,
        0.,                  0., 1.
    );

    transformation_x * transformation_y * transformation_z
}