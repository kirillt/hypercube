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

pub fn rotation_matrix_x(angle: CoordFloat) -> Matrix4 {
    rotation_matrix(vec![angle, 0., 0., 0., 0., 0.])
}

pub fn rotation_matrix_y(angle: CoordFloat) -> Matrix4 {
    rotation_matrix(vec![0., angle, 0., 0., 0., 0.])
}

pub fn rotation_matrix_z(angle: CoordFloat) -> Matrix4 {
    rotation_matrix(vec![0., 0., angle, 0., 0., 0.])
}

pub fn rotation_matrix(angles: Vec<CoordFloat>) -> Matrix4 {
    assert!(angles.len() == 6);
    let angle_wx = angles[0];
    let angle_wy = angles[1];
    let angle_wz = angles[2];
    let angle_xy = angles[3];
    let angle_xz = angles[4];
    let angle_yz = angles[5];

    let transformation_wx = Matrix4::new(
        1.,  0.,             0.,                   0.,
        0.,  1.,             0.,                   0.,
        0.,  0., angle_wx.cos(), -1. * angle_wx.sin(),
        0.,  0., angle_wx.sin(),       angle_wx.cos()
    ); //yz

    let transformation_wy = Matrix4::new(
        1.,             0., 0.,                   0.,
        0., angle_wy.cos(), 0., -1. * angle_wy.sin(),
        0.,             0., 1.,                   0.,
        0., angle_wy.sin(), 0.,       angle_wy.cos()
    ); //xz

    let transformation_wz = Matrix4::new(
        1.,             0.,                   0., 0.,
        0., angle_wz.cos(), -1. * angle_wz.sin(), 0.,
        0., angle_wz.sin(),       angle_wz.cos(), 0.,
        0.,             0.,                   0., 1.,
    ); //xy

    let transformation_xy = Matrix4::new(
        angle_xy.cos(), 0., 0., -1. * angle_xy.sin(),
                    0., 1., 0.,                   0.,
                    0., 0., 1.,                   0.,
        angle_xy.sin(), 0., 0.,       angle_xy.cos()
    ); //wz

    let transformation_xz = Matrix4::new(
        angle_xz.cos(), 0., -1. * angle_xz.sin(), 0.,
                    0., 1.,                   0., 0.,
        angle_xz.sin(), 0.,       angle_xz.cos(), 0.,
                    0., 0.,                   0., 1.
    ); //wy

    let transformation_yz = Matrix4::new(
        angle_yz.cos(), -1. * angle_yz.sin(), 0., 0.,
        angle_yz.sin(),       angle_yz.cos(), 0., 0.,
                    0.,                   0., 1., 0.,
                    0.,                   0., 0., 1.
    ); //wx


    transformation_wx * transformation_wy * transformation_wz *
        transformation_xy * transformation_xz *
        transformation_yz
}