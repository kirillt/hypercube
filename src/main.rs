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
mod time;

use core::*;
use motion::{Animated, Rotation};
use model::wireframe::{self, Joint};
use model::combine;
use model::figures;

use state::*;

fn main() {
    let pyramid = figures::tetrahedron(unit_x(), unit_y(), unit_z(), unit_xyz(), blue(), red())
        .scale_eq(0.8)
        .shift_x(0.1)
        .shift_y(0.1)
        .shift_z(0.1);
    let pyramid = Rotation::d3(pyramid, vec![PI / 2., PI / 3., PI / 5.], 60);

    let frustum = combine::tower(
            figures::triangle(unit_x(), unit_y(), unit_z(), red()),
            figures::triangle(unit_x() * 3., unit_y() * 3., unit_z() * 3., green()))
        .scale_eq(0.2)
        .shift_y(2.0)
        .shift_z(1.0)
        .shift_z(0.5);

    let objects = Rotation::d3(motion::compose(pyramid, frustum),
                               vec![PI / 2., PI / 3., PI / 5.], 60);

    let cage = combine::merge_group(vec![
        wireframe::square_xy(0.07, 3.3, (blue(), green(), red(), yellow()), 6, Joint::smooth(0.15)),
        wireframe::square_yz(0.07, 3.3, (blue(), green(), red(), yellow()), 6, Joint::smooth(0.15)),
        wireframe::square_zx(0.07, 3.3, (blue(), green(), red(), yellow()), 6, Joint::smooth(0.15))
    ]);

    let axis = combine::merge_group(vec![
        wireframe::stick_x(0.01, 32., (black(), red()), 6),
        wireframe::stick_y(0.01, 32., (black(), blue()), 6),
        wireframe::stick_z(0.01, 32., (black(), green()), 6),
    ]);

    let scene = motion::compose(
        combine::merge(axis, cage),
        objects);

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
        js_assert(remainder == 0, false, "triangulation looks broken".to_string());
    }

    js! {
        window.state = {};
        window.state.pause = false;
        window.state.rotation = true;
    }

    run(scene, 40., [0., 0., -6.],
        [15. * PI, 10. * PI, 5. * PI]);
}