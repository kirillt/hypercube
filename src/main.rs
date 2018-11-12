#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb_derive;

extern crate itertools;

mod webgl_rendering_context;

mod shaders;
mod buffer;
mod canvas;
mod state;

mod composition;
mod figures;
mod vector;
mod render;
mod shapes;
mod prism;

use render::*;
use vector::*;
use composition::*;

use state::*;

use std::f32::consts::PI;

fn main() {
    let pyramid = shapes::Tetrahedron {
        base: figures::Triangle {
            a: vector::UNIT_X,
            b: vector::UNIT_Y,
            c: vector::UNIT_Z,

            color: render::BLUE
        },

        peak: vector::UNIT
    };

    let prism = prism::build(
        figures::Triangle {
            a: vector::UNIT_X,
            b: vector::UNIT_Y,
            c: vector::UNIT_Z,

            color: GREEN
        },

        figures::Triangle {
            a: vector::UNIT_X.shift_x(2.0),
            b: vector::UNIT_Y.shift_x(2.0),
            c: vector::UNIT_Z.shift_x(2.0),

            color: RED
        }
    ).scale_eq(0.3)
     .shift_y(2.0)
     .shift_z(1.0);

    let scene = Composition {
        first: pyramid,
        second: prism
    };

    let n = scene.positions().len() as u16;
    let m = scene.indices().iter().cloned().fold(0, u16::max);
    js! {
        console.log("size of scene: " + @{n});
        console.log("max index: " + @{m});
    }

    run(scene,
        40., [0., 0., -6.],
        [15. * PI, 10. * PI, 5. * PI]);
}