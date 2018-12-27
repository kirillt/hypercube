use core::*;

use model::combine;
use model::figures;
use model::transform::*;
use model::polar::*;

use model::Snapshot;

pub struct Joint {
    radius: CoordFloat,
    detailing: u16
}

impl Joint {
    pub fn none() -> Option<Self> {
        None
    }

    pub fn rough(radius: CoordFloat) -> Option<Self> {
        Some(Joint { radius: radius, detailing: 6 })
    }
    pub fn normal(radius: CoordFloat) -> Option<Self> {
        Some(Joint { radius: radius, detailing: 12 })
    }
    pub fn smooth(radius: CoordFloat) -> Option<Self> {
        Some(Joint { radius: radius, detailing: 24 })
    }
}

pub type Vertex = (Point, Color);

pub fn stick_between(a: Vertex, b: Vertex,
                     gauge: CoordFloat,
                     detailing: u16) -> Snapshot {
    let offset: Vector = a.0.coords;
    let polar = Polar::from_descartes(b.0.coords - a.0.coords);

    js_log(format!("input: {:?}", b.0 - a.0));
    js_log(format!("length = {}, angles: [{},{},{}]", polar.ro, polar.alpha, polar.beta, polar.gamma));
    js_log(format!("output: {:?}", polar.to_descartes()));

    stick(offset, polar.ro, polar.angles(), gauge, (a.1, b.1), detailing)
}

pub fn stick(offset: Vector, length: CoordFloat, angles: Angles,
             gauge: CoordFloat, colors: (Color, Color), detailing: u16) -> Snapshot {
    stick_x(length, gauge, colors, detailing)
        .shift_x(length / 2.)
        .rotate(vec![0., 0., angles.gamma, angles.alpha, angles.beta, 0.])
        .shift(offset)
}

pub fn stick_x(length: CoordFloat, gauge: CoordFloat, colors: (Color, Color), detailing: u16) -> Snapshot {
    stick_z(length, gauge, colors, detailing).turn_around_y(PI / 2.)
}

pub fn stick_y(length: CoordFloat, gauge: CoordFloat, colors: (Color, Color), detailing: u16) -> Snapshot {
    stick_z(length, gauge, colors, detailing).turn_around_x(PI / 2.)
}

pub fn stick_z(length: CoordFloat, gauge: CoordFloat, colors: (Color, Color), detailing: u16) -> Snapshot {
    let offset = length / 2.;
    combine::tower(
        figures::circle_xy(origin(), gauge, colors.1, detailing).shift_z(-offset),
        figures::circle_xy(origin(), gauge, colors.0, detailing).shift_z(offset))
}

pub fn square_xy(side: CoordFloat, gauge: CoordFloat, colors: (Color, Color, Color, Color),
                 detailing: u16, balls_radius: Option<Joint>) -> Snapshot {
    let left = stick_y(side, gauge, (colors.3, colors.0), detailing);
    let right = stick_y(side, gauge, (colors.2, colors.1), detailing);
    let top = stick_x(side, gauge, (colors.0, colors.1), detailing);
    let bottom = stick_x(side, gauge, (colors.3, colors.2), detailing);

    let offset = side / 2.;

    let mut result = balls_radius
        .map(|joint| vec![
            figures::sphere_xyz(Point::new(0., -offset,  offset, 0.), joint.radius, colors.0, joint.detailing),
            figures::sphere_xyz(Point::new(0.,  offset,  offset, 0.), joint.radius, colors.1, joint.detailing),
            figures::sphere_xyz(Point::new(0.,  offset, -offset, 0.), joint.radius, colors.2, joint.detailing),
            figures::sphere_xyz(Point::new(0., -offset, -offset, 0.), joint.radius, colors.3, joint.detailing),
        ])
        .unwrap_or(vec![]);

    result.push(left.shift_x(-offset));
    result.push(right.shift_x(offset));
    result.push(top.shift_y(offset));
    result.push(bottom.shift_y(-offset));

    combine::merge_group(result)
}

pub fn square_yz(side: CoordFloat, gauge: CoordFloat, colors: (Color, Color, Color, Color),
                 detailing: u16, balls_radius: Option<Joint>) -> Snapshot {
    square_xy(side, gauge, colors, detailing, balls_radius).turn_around_y(PI / 2.)
}

pub fn square_zx(side: CoordFloat, gauge: CoordFloat, colors: (Color, Color, Color, Color),
                 detailing: u16, balls_radius: Option<Joint>) -> Snapshot {
    square_xy(side, gauge, colors, detailing, balls_radius).turn_around_x(PI / 2.)
}