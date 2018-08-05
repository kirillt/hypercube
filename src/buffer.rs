use stdweb::web::TypedArray;

use webgl_rendering_context::{
    WebGLRenderingContext as gl,
    WebGLBuffer
};

pub fn array(context: &gl, values: &[f32]) -> WebGLBuffer {
    let data = TypedArray::<f32>::from(values).buffer();

    let buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&data), gl::STATIC_DRAW);

    buffer
}

pub fn indices(context: &gl, indices: &[u16]) -> WebGLBuffer {
    let indices = TypedArray::<u16>::from(indices).buffer();

    let index_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    context.buffer_data_1(gl::ELEMENT_ARRAY_BUFFER, Some(&indices), gl::STATIC_DRAW);

    index_buffer
}