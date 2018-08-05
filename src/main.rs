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

use webgl_rendering_context::{
    WebGLRenderingContext as gl,
    WebGLProgram
};

use state::*;

fn main() {
    stdweb::initialize();

    let canvas = canvas::establish();
    let context: gl = canvas.get_context().unwrap();

    let shaders: WebGLProgram = shaders::establish(&context);

    shaders::associate(&context, &shaders, "position",
        buffer::array(&context, &[
            -1.,-1.,-1.,  1.,-1.,-1.,  1., 1.,-1., -1., 1.,-1.,
            -1.,-1., 1.,  1.,-1., 1.,  1., 1., 1., -1., 1., 1.,
            -1.,-1.,-1., -1., 1.,-1., -1., 1., 1., -1.,-1., 1.,
            1.,-1.,-1.,  1., 1.,-1.,  1., 1., 1.,  1.,-1., 1.,
            -1.,-1.,-1., -1.,-1., 1.,  1.,-1., 1.,  1.,-1.,-1.,
            -1., 1.,-1., -1., 1., 1.,  1., 1., 1.,  1., 1.,-1.,
        ][..]));

    shaders::associate(&context, &shaders, "color",
       buffer::array(&context, &[
           5.,3.,7., 5.,3.,7., 5.,3.,7., 5.,3.,7.,
           1.,1.,3., 1.,1.,3., 1.,1.,3., 1.,1.,3.,
           0.,0.,1., 0.,0.,1., 0.,0.,1., 0.,0.,1.,
           1.,0.,0., 1.,0.,0., 1.,0.,0., 1.,0.,0.,
           1.,1.,0., 1.,1.,0., 1.,1.,0., 1.,1.,0.,
           0.,1.,0., 0.,1.,0., 0.,1.,0., 0.,1.,0.
       ][..]));

    context.use_program(Some(&shaders));


    let mov_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];
    let mut view_matrix = [1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.];

    // translating z
    view_matrix[14] -= 6.; //zoom


    let p_matrix = context.get_uniform_location(&shaders, "Pmatrix").unwrap();
    let v_matrix = context.get_uniform_location(&shaders, "Vmatrix").unwrap();
    let m_matrix = context.get_uniform_location(&shaders, "Mmatrix").unwrap();

    let index_buffer = buffer::indices(&context, &[
         0, 1, 2,   0, 2, 3,   4, 5, 6,   4, 6, 7,
         8, 9,10,   8,10,11,  12,13,14,  12,14,15,
        16,17,18,  16,18,19,  20,21,22,  20,22,23
    ][..]);

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
    }));

    state.borrow_mut().animate(0., state.clone());

    stdweb::event_loop();
}