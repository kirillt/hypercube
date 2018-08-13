use buffer;

use render::Renderable;

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

pub fn bind<R: Renderable>(context: &gl, program: &WebGLProgram, scene: &R) {
    bind_attribute(context, program, "position", buffer::array(&context, &scene.positions_flat()));
    bind_attribute(context, program, "color", buffer::array(&context, &scene.colors_flat()));
}

fn bind_attribute(context: &gl, program: &WebGLProgram, attribute: &str, buffer: WebGLBuffer) {
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&buffer));
    let attribute_location = context.get_attrib_location(program, attribute) as u32;
    context.vertex_attrib_pointer(attribute_location, 3, gl::FLOAT, false, 0, 0) ;
    context.enable_vertex_attrib_array(attribute_location);
}