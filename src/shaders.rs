use core::*;

use stdweb::web::TypedArray;

use webgl_rendering_context::{
    WebGLRenderingContext as gl,
    WebGLProgram,
    WebGLBuffer
};

const VERTEX_SHADER: &str = r#"
        attribute vec3 position;
        uniform mat4 Pmatrix;
        uniform mat4 Vmatrix;
        uniform mat4 Mmatrix;
        attribute vec3 color;
        varying vec3 vColor;

        void main() {
            gl_PointSize = 10.;
            gl_Position = Pmatrix*Vmatrix*Mmatrix*vec4(position, 1.);
            vColor = color;
        }
    "#;

const FRAGMENT_SHADER: &str = r#"
        precision mediump float;
        varying vec3 vColor;

        void main() {
            gl_FragColor = vec4(vColor, 1.);
        }
    "#;

pub fn establish(context: &gl) -> WebGLProgram {
    let vert_shader = context.create_shader(gl::VERTEX_SHADER).unwrap();
    context.shader_source(&vert_shader, VERTEX_SHADER);
    context.compile_shader(&vert_shader);

    let frag_shader = context.create_shader(gl::FRAGMENT_SHADER).unwrap();
    context.shader_source(&frag_shader, FRAGMENT_SHADER);
    context.compile_shader(&frag_shader);

    let shader_program: WebGLProgram = context.create_program().unwrap();
    context.attach_shader(&shader_program, &vert_shader);
    context.attach_shader(&shader_program, &frag_shader);
    context.link_program(&shader_program);

    shader_program
}

pub fn bind(context: &gl, program: &WebGLProgram, positions: Refs<Vec<Point>>, colors: Refs<Vec<Color>>) {
    //js! { window.coordinates = performance.now(); }
    let coordinates = flatten_coordinates(positions);
    //js! { console.log("binding coordinates: " + (performance.now() - window.coordinates)); }

    //js! { window.colors = performance.now(); }
    let components = flatten_components(colors);
    //js! { console.log("binding colors: " + (performance.now() - window.colors)); }

    bind_attribute(context, program, "position", array_buffer(&context, &coordinates[..]));
    bind_attribute(context, program, "color", array_buffer(&context, &components[..]));
}

fn bind_attribute(context: &gl, program: &WebGLProgram, attribute: &str, buffer: WebGLBuffer) {
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&buffer));
    let attribute_location = context.get_attrib_location(program, attribute) as u32;
    context.vertex_attrib_pointer(attribute_location, 3, gl::FLOAT, false, 0, 0) ;
    context.enable_vertex_attrib_array(attribute_location);
}

pub fn array_buffer(context: &gl, values: &[f32]) -> WebGLBuffer {
    let data = TypedArray::<f32>::from(values).buffer();

    let buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&buffer));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&data), gl::STATIC_DRAW);

    buffer
}

pub fn indices_buffer(context: &gl, indices: &[u16]) -> WebGLBuffer {
    let indices = TypedArray::<u16>::from(indices).buffer();

    let index_buffer = context.create_buffer().unwrap();
    context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    context.buffer_data_1(gl::ELEMENT_ARRAY_BUFFER, Some(&indices), gl::STATIC_DRAW);

    index_buffer
}