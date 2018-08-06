#![feature(iterator_flatten)]

#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb_derive;

use std::rc::Rc;
use std::cell::RefCell;

mod webgl_rendering_context;

mod state;
mod transformations;
mod shaders;
mod buffer;
mod canvas;

mod vector;
mod render;
mod figures;
mod shapes;

use render::Renderable;

use webgl_rendering_context::{
    WebGLRenderingContext as gl,
    WebGLProgram
};

use stdweb::web::{
    document, IParentNode, HtmlElement
};

use stdweb::unstable::TryInto;

use state::*;

fn main() {
    stdweb::initialize();

    let canvas = canvas::establish();
    let context: gl = canvas.get_context().unwrap();

    let shaders: WebGLProgram = shaders::establish(&context);

    let object = shapes::Tetrahedron {
        base: figures::Triangle {
            a: vector::UNIT_X,
            b: vector::UNIT_Y,
            c: vector::UNIT_Z,

            color: render::BLUE
        },

        peak: vector::UNIT
    };

    shaders::bind(&context, &shaders, &object);
    context.use_program(Some(&shaders));

    let mov_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];
    let mut view_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];

    // translating z
    view_matrix[14] -= 6.; //zoom


    let p_matrix = context.get_uniform_location(&shaders, "Pmatrix").unwrap();
    let v_matrix = context.get_uniform_location(&shaders, "Vmatrix").unwrap();
    let m_matrix = context.get_uniform_location(&shaders, "Mmatrix").unwrap();

    let (index_buffer, size) = {
        let indices = object.indices();
        (buffer::indices(&context, &indices), indices.len() as i32)
    };

    let fps_div: HtmlElement = document().query_selector("#fps").unwrap().unwrap().try_into().unwrap();

    let prev = ::std::f64::NAN;
    let state = Rc::new(RefCell::new(State {
        time_old: 0.0,
        mov_matrix,
        view_matrix,
        canvas,
        context,
        p_matrix,
        v_matrix,
        m_matrix,
        index_buffer,
        size,

        fps_div,
        prev,
        frames: 0
    }));

    state.borrow_mut().animate(0., state.clone());

    stdweb::event_loop();
}