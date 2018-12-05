use core::*;

use model::combine;
use model::figures;
use model::transform::*;
use model::Snapshot;

pub fn stick_x(gauge: CoordFloat, length: CoordFloat, colors: (Color, Color), detailing: u16) -> Snapshot {
    stick_z(gauge, length, colors, detailing).turn_around_y(PI / 2.)
}

pub fn stick_y(gauge: CoordFloat, length: CoordFloat, colors: (Color, Color), detailing: u16) -> Snapshot {
    stick_z(gauge, length, colors, detailing).turn_around_x(PI / 2.)
}

pub fn stick_z(gauge: CoordFloat, length: CoordFloat, colors: (Color, Color), detailing: u16) -> Snapshot {
    let offset = length / 2.;
    combine::tower(
        figures::circle_xy(origin(), gauge, colors.1, detailing).shift_z(- offset),
        figures::circle_xy(origin(), gauge, colors.0, detailing).shift_z(offset))
}