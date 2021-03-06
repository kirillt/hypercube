use core::*;
use motion::animated::Animated;
use model::transform::*;

use std::rc::Rc;

#[derive(Clone)]
pub struct Snapshot {
    pub positions: Rc<Vec<Point>>,
    pub colors: Rc<Vec<Color>>,
    pub indices: Vec<u16>,
    pub size: u16
}

impl Animated for Snapshot {
    fn positions(&self, _time: usize) -> Refs<Vec<Point>> {
        vec![self.positions.clone()]
    }

    fn colors(&self, _time: usize) -> Refs<Vec<Color>> {
        vec![self.colors.clone()]
    }

    fn indices(&self) -> Vec<u16> {
        self.indices.clone()
    }

    fn size(&self) -> u16 {
        self.size
    }
}

#[allow(dead_code)]
impl Snapshot {
    pub fn scale_eq(self, factor: CoordFloat) -> Snapshot {
        self.scale(Vector::new(factor, factor, factor, factor))
    }
    pub fn scale_w(self, factor: CoordFloat) -> Snapshot {
        self.scale(Vector::new(factor, 1., 1., 1.))
    }
    pub fn scale_x(self, factor: CoordFloat) -> Snapshot {
        self.scale(Vector::new(1., factor, 1., 1.))
    }
    pub fn scale_y(self, factor: CoordFloat) -> Snapshot {
        self.scale(Vector::new(1., 1., factor, 1.))
    }
    pub fn scale_z(self, factor: CoordFloat) -> Snapshot {
        self.scale(Vector::new(1., 1., 1., factor))
    }
    pub fn scale(self, factors: Vector) -> Snapshot {
        assert!(factors.len() == 4);
        self.map_positions(&|point|
            point.coords.component_mul(&factors).into())
    }

    pub fn shift_w(self, offset: CoordFloat) -> Snapshot {
        self.shift(Vector::new(offset, 0., 0., 0.))
    }
    pub fn shift_x(self, offset: CoordFloat) -> Snapshot {
        self.shift(Vector::new(0., offset, 0., 0.))
    }
    pub fn shift_y(self, offset: CoordFloat) -> Snapshot {
        self.shift(Vector::new(0., 0., offset, 0.))
    }
    pub fn shift_z(self, offset: CoordFloat) -> Snapshot {
        self.shift(Vector::new(0., 0., 0., offset))
    }
    pub fn shift(self, offset: Vector) -> Snapshot {
        self.map_positions(&|point| point + offset)
    }

    pub fn turn_around_x(self, angle: CoordFloat) -> Snapshot {
        let rotation = rotation_matrix_x(angle);
        self.map_positions(&|point| rotation * point)
    }

    pub fn turn_around_y(self, angle: CoordFloat) -> Snapshot {
        let rotation = rotation_matrix_y(angle);
        self.map_positions(&|point| rotation * point)
    }

    pub fn turn_around_z(self, angle: CoordFloat) -> Snapshot {
        let rotation = rotation_matrix_z(angle);
        self.map_positions(&|point| rotation * point)
    }

    pub fn rotate(self, angles: Vec<CoordFloat>) -> Snapshot {
        let rotation = rotation_matrix(angles);
        self.map_positions(&|point| rotation * point)
    }

    pub fn map_positions(self, transform: &Fn(Point) -> Point) -> Snapshot {
        let chunks = Rc::try_unwrap(self.positions)
            .unwrap();

        let positions = chunks
            .into_iter()
            .map(|point| transform(point))
            .collect();

        Snapshot {
            positions: Rc::new(positions),
            colors: self.colors,
            indices: self.indices,
            size: self.size
        }
    }

    pub fn duplicate(self) -> Snapshot {
        Snapshot {
            positions: Rc::new((*self.positions).clone()),
            colors: Rc::new((*self.colors).clone()),
            indices: self.indices.clone(),
            size: self.size
        }
    }
}