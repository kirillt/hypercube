use core::*;
use motion::animated::Animated;

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
        self.scale(Vector::new(factor, factor, factor))
    }
    pub fn scale_x(self, factor: CoordFloat) -> Snapshot {
        self.scale(Vector::new(factor, 1., 1.))
    }
    pub fn scale_y(self, factor: CoordFloat) -> Snapshot {
        self.scale(Vector::new(1., factor, 1.))
    }
    pub fn scale_z(self, factor: CoordFloat) -> Snapshot {
        self.scale(Vector::new(1., 1., factor))
    }
    pub fn scale(self, factors: Vector) -> Snapshot {
        self.map_positions(&|point|
            point.coords.component_mul(&factors).into())
    }

    pub fn shift_x(self, offset: CoordFloat) -> Snapshot {
        self.shift(Vector::new(offset, 0., 0.))
    }
    pub fn shift_y(self, offset: CoordFloat) -> Snapshot {
        self.shift(Vector::new(0., offset, 0.))
    }
    pub fn shift_z(self, offset: CoordFloat) -> Snapshot {
        self.shift(Vector::new(0., 0., offset))
    }
    pub fn shift(self, offset: Vector) -> Snapshot {
        self.map_positions(&|point| point + offset)
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
}