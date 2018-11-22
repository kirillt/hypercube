#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb_derive;

extern crate itertools;
extern crate nalgebra;

mod webgl_rendering_context;

mod core;
mod model;
mod motion;

mod shaders;
mod canvas;
mod state;

use core::*;
use motion::{Animated, Rotation};
use model::figures;

use state::*;

use std::f32::consts::PI;

fn main() {
    let pyramid = figures::tetrahedron(unit_x(), unit_y(), unit_z(), unit_xyz(), blue(), red());
    let pyramid = Rotation::new(pyramid, vec![PI / 2., PI / 3., PI / 5., 0., 0., 0.], 100);

    let prism = figures::prism(
            figures::triangle(unit_x(), unit_y(), unit_z(), green()),
            figures::triangle(unit_x() * 3., unit_y() * 3., unit_z() * 3., red()))
        .scale_eq(0.3)
        .shift_y(2.0)
        .shift_z(1.0);

    let scene = motion::compose(pyramid, prism);

    {
        //debug
        let n = scene.size();
        let m = scene.indices().iter().cloned().fold(0, u16::max);
        js! {
            console.log("points of scene: " + @{format!("{:?}", scene.positions(0))});
            console.log("size of scene: " + @{n});
            console.log("max index: " + @{m});
        }
    }

    run(scene,
        40., [0., 0., -6.],
        [15. * PI, 10. * PI, 5. * PI]);
}