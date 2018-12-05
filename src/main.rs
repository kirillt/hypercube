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
    let pyramid = Rotation::d3(pyramid, vec![PI / 2., PI / 3., PI / 5.], 60);

    let frustum = figures::tower(
            figures::triangle(unit_x(), unit_y(), unit_z(), red()),
            figures::triangle(unit_x() * 3., unit_y() * 3., unit_z() * 3., green()))
        .scale_eq(0.3)
        .shift_y(2.0)
        .shift_z(1.0);

    let scene = motion::compose(pyramid, frustum);
    let scene = Rotation::d3(scene, vec![PI / 2., PI / 3., PI / 5.], 60);

    let ball = figures::sphere_xyz_colored(origin(), 1., 24,
                                           blue(), yellow(), red())
        .scale_eq(0.15)
        .shift_x(-3.0)
        .shift_y(1.0)
        .shift_z(-2.0);
    let ball = Rotation::d3(ball, vec![PI / 7., PI / 5., PI / 3.], 60);

    let scene = motion::compose(scene, ball);

    {
        //debug
        let total = scene.size();
        let max_index = scene.indices().iter().cloned().fold(0, u16::max);
        let triangles = (scene.indices().len() / 3) as u32;
        let remainder = (scene.indices().len() % 3) as u32;
        js! {
            console.log("points of scene: " + @{format!("{:?}", scene.positions(0))});
            console.log("size of scene: " + @{total});
            console.log("max index: " + @{max_index});
            console.log("triangles: " + @{triangles});
            console.log("non matched points: " + @{remainder});
        }
        js_assert(remainder == 0, "triangulation looks broken".to_string());
    }

    run(scene,
        40., [0., 0., -6.],
//        [0., 0., 0.]);
        [15. * PI, 10. * PI, 5. * PI]);
}