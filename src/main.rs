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

mod model;
use model::vector;
use model::figures;
use model::shapes;
use model::prism;
use model::render;
use model::render::Renderable;

mod motion;
use motion::Constant;
use motion::animated::Animated;

use render::*;
use vector::*;

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

    let scene = motion::compose(Constant::new(pyramid), Constant::new(prism));

    {
        //debug
        let first_draw = scene.calculate(0);
        let n = first_draw.positions().len() as u16;
        let m = first_draw.indices().iter().cloned().fold(0, u16::max);
        js! {
            console.log("size of scene: " + @{n});
            console.log("max index: " + @{m});
        }
    }

    run(scene,
        40., [0., 0., -6.],
        [15. * PI, 10. * PI, 5. * PI]);
}