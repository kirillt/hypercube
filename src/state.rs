use std::rc::Rc;
use std::cell::RefCell;

use stdweb::web::html_element::CanvasElement;

use stdweb::web::{
    window,
};

use webgl_rendering_context::{
    WebGLRenderingContext as gl,
    WebGLUniformLocation,
    WebGLBuffer
};

use transformations::*;

pub struct State {
    pub time_old: f64,
    pub mov_matrix: [f32; 16],
    pub view_matrix: [f32; 16],
    pub canvas: CanvasElement,
    pub context: gl,
    pub p_matrix: WebGLUniformLocation,
    pub v_matrix: WebGLUniformLocation,
    pub m_matrix: WebGLUniformLocation,
    pub index_buffer: WebGLBuffer,
}

impl State {
    pub fn animate(&mut self, time: f64, rc: Rc<RefCell<Self>>) {
        let dt = (time - self.time_old) as f32;
        rotate_z(&mut self.mov_matrix, dt*0.0007);//time
        rotate_y(&mut self.mov_matrix, dt*0.0002);
        rotate_x(&mut self.mov_matrix, dt*0.0003);
        self.time_old = time;

        self.context.enable(gl::DEPTH_TEST);
        self.context.depth_func(gl::LEQUAL);
        self.context.clear_color(0.5, 0.5, 0.5, 0.9);
        self.context.clear_depth(1.0);

        let (w, h) = (self.canvas.width(), self.canvas.height());
        let proj_matrix = get_projection(40., (w as f32)/(h as f32), 1., 100.);

        self.context.viewport(0, 0, w as i32, h as i32);
        self.context.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.context.uniform_matrix4fv(Some(&self.p_matrix), false, &proj_matrix[..]);
        self.context.uniform_matrix4fv(Some(&self.v_matrix), false, &self.view_matrix[..]);
        self.context.uniform_matrix4fv(Some(&self.m_matrix), false, &self.mov_matrix[..]);
        self.context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        self.context.draw_elements(gl::TRIANGLES, 36, gl::UNSIGNED_SHORT, 0);

        window().request_animation_frame(move |time| {
            rc.borrow_mut().animate(time, rc.clone());
        });
    }
}