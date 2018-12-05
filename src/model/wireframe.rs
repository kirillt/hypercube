use core::*;

use model::combine;
use model::figures;
use model::transform::*;
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

pub fn stick_x(gauge: CoordFloat, length: CoordFloat, colors: (Color, Color), detailing: u16) -> Snapshot {
    stick_z(gauge, length, colors, detailing).turn_around_y(PI / 2.)
}

pub fn stick_y(gauge: CoordFloat, length: CoordFloat, colors: (Color, Color), detailing: u16) -> Snapshot {
    stick_z(gauge, length, colors, detailing).turn_around_x(PI / 2.)
}

pub fn stick_z(gauge: CoordFloat, length: CoordFloat, colors: (Color, Color), detailing: u16) -> Snapshot {
    let offset = length / 2.;
    combine::tower(
        figures::circle_xy(origin(), gauge, colors.1, detailing).shift_z(-offset),
        figures::circle_xy(origin(), gauge, colors.0, detailing).shift_z(offset))
}

pub fn square_xy(gauge: CoordFloat, side: CoordFloat, colors: (Color, Color, Color, Color),
                 detailing: u16, balls_radius: Option<Joint>) -> Snapshot {
    let left = stick_y(gauge, side, (colors.3, colors.0), detailing);
    let right = stick_y(gauge, side, (colors.2, colors.1), detailing);
    let top = stick_x(gauge, side, (colors.0, colors.1), detailing);
    let bottom = stick_x(gauge, side, (colors.3, colors.2), detailing);

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

pub fn square_yz(gauge: CoordFloat, side: CoordFloat, colors: (Color, Color, Color, Color),
                 detailing: u16, balls_radius: Option<Joint>) -> Snapshot {
    square_xy(gauge, side, colors, detailing, balls_radius).turn_around_y(PI / 2.)
}

pub fn square_zx(gauge: CoordFloat, side: CoordFloat, colors: (Color, Color, Color, Color),
                 detailing: u16, balls_radius: Option<Joint>) -> Snapshot {
    square_xy(gauge, side, colors, detailing, balls_radius).turn_around_x(PI / 2.)
}