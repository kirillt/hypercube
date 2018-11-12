use render::Renderable;
use shaders;
use buffer;
use canvas;

use std::rc::Rc;
use std::cell::RefCell;

use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{
    window, document,
    IParentNode, INode,
    HtmlElement
};

use webgl_rendering_context::{
    WebGLRenderingContext as gl,
    WebGLUniformLocation,
    WebGLProgram,
    WebGLBuffer
};

pub struct RenderingContext<R: Renderable> {
    canvas: CanvasElement,
    context: gl,
    shaders: WebGLProgram,

    scene: R,
    rotation: [f32; 3],
    fov: f32
}

pub struct RenderingState {
    p_matrix: WebGLUniformLocation,
    v_matrix: WebGLUniformLocation,
    m_matrix: WebGLUniformLocation,
    model_matrix: [f32; 16],
    view_matrix: [f32; 16],

    index_buffer: WebGLBuffer,
    size: i32,

    fps_div: HtmlElement,
    last_drawed: Option<usize>,
    frames: usize
}

pub fn run<R: Renderable + 'static>(scene: R,
                          fov: f32, zoom: [f32; 3],
                          rotation: [f32; 3]) {
    let canvas = canvas::establish();
    let context: gl = canvas.get_context().unwrap();
    let shaders: WebGLProgram = shaders::establish(&context);

    let ctx = RenderingContext {
        canvas,
        context,
        shaders,

        scene,
        rotation,
        fov,
    };

    let state = RenderingState::new(&ctx, zoom);
    let rc = Rc::new(RefCell::new(state));

    rc.borrow_mut().animate(ctx, rc.clone(), 0.);
}

impl RenderingState {
    pub fn new<R: Renderable>(ctx: &RenderingContext<R>, zoom: [f32; 3]) -> Self {
        let (index_buffer, size) = {
            let indices = ctx.scene.indices();
            (buffer::indices(&ctx.context, &indices), indices.len() as i32)
        };

        let fps_div: HtmlElement = document()
            .query_selector("#fps")
            .unwrap().unwrap()
            .try_into()
            .unwrap();

        RenderingState {
            p_matrix: ctx.context.get_uniform_location(&ctx.shaders, "Pmatrix").unwrap(),
            v_matrix: ctx.context.get_uniform_location(&ctx.shaders, "Vmatrix").unwrap(),
            m_matrix: ctx.context.get_uniform_location(&ctx.shaders, "Mmatrix").unwrap(),
            model_matrix: ID_MATRIX,
            view_matrix: zoom_matrix(zoom),
            index_buffer,
            size,

            fps_div,
            last_drawed: None,
            frames: 0,
        }
    }

    fn animate<R: Renderable + 'static>(&mut self, ctx: RenderingContext<R>, rc: Rc<RefCell<Self>>, time: f64) {
        let time = time as usize;

        match self.last_drawed {
            Some(last_drawed) => {
                let delta = time - last_drawed;
                if delta > FPS_DELTA_MS {
                    let fps = 1000 * self.frames / delta;
                    self.fps_div.set_text_content(fps.to_string().as_str());
                    self.last_drawed = Some(time);
                    self.frames = 0;
                }
            },
            None => self.last_drawed = Some(time)
        }
        self.frames += 1;

        //todo: additional model rotation can be done here
        shaders::bind(&ctx.context, &ctx.shaders, &ctx.scene);
        ctx.context.use_program(Some(&ctx.shaders));

        let phase = (time % TIME_LOOP_MS) as f32 / (TIME_LOOP_MS as f32);

        let mut model_matrix = self.model_matrix.clone();
        rotate_x(&mut model_matrix, phase * ctx.rotation[0]);
        rotate_y(&mut model_matrix, phase * ctx.rotation[1]);
        rotate_z(&mut model_matrix, phase * ctx.rotation[2]);

        ctx.context.enable(gl::DEPTH_TEST);
        ctx.context.depth_func(gl::LEQUAL);
        ctx.context.clear_color(0.5, 0.5, 0.5, 0.9);
        ctx.context.clear_depth(1.0);

        let (width, height) = (ctx.canvas.width(), ctx.canvas.height());
        let projection_matrix = projection(ctx.fov, (width as f32) / (height as f32), 1., 100.);

        ctx.context.viewport(0, 0, width as i32, height as i32);
        ctx.context.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        ctx.context.uniform_matrix4fv(Some(&self.p_matrix), false, &projection_matrix[..]);
        ctx.context.uniform_matrix4fv(Some(&self.v_matrix), false, &self.view_matrix[..]);
        ctx.context.uniform_matrix4fv(Some(&self.m_matrix), false, &model_matrix[..]);
        ctx.context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        ctx.context.draw_elements(gl::TRIANGLES, self.size, gl::UNSIGNED_SHORT, 0);

        window().request_animation_frame(move |time| {
            rc.borrow_mut().animate(ctx, rc.clone(), time);
        });
    }
}

const FPS_DELTA_MS: usize = 300;
const TIME_LOOP_MS: usize = 60_000;

const ID_MATRIX: [f32; 16] = [1.,0.,0.,0.,  0.,1.,0.,0.,  0.,0.,1.,0.,  0.,0.,0.,1.];

fn zoom_matrix(zoom: [f32; 3]) -> [f32; 16] {
    let mut matrix = ID_MATRIX.clone();
    matrix[12] = zoom[0];
    matrix[13] = zoom[1];
    matrix[14] = zoom[2];
    matrix
}

fn projection(angle: f32, frame_ratio: f32, z_min: f32, z_max: f32) -> [f32; 16] {
    let angle = (angle * 0.5).to_radians().tan();
    return [
        0.5 / angle, 0., 0., 0.,
        0., 0.5 * frame_ratio / angle, 0., 0.,
        0., 0., - (z_max + z_min) / (z_max - z_min), -1.,
        0., 0., -2. * z_max * z_min / (z_max - z_min), 0.
    ];
}

fn rotate_x(m: &mut [f32; 16], angle: f32) {
    let (sin, cos) = angle.sin_cos();
    let (mv1, mv5, mv9) = (m[1], m[5], m[9]);

    m[1]  = m[1]  * cos - m[2]  * sin;
    m[5]  = m[5]  * cos - m[6]  * sin;
    m[9]  = m[9]  * cos - m[10] * sin;

    m[2]  = m[2]  * cos + mv1 * sin;
    m[6]  = m[6]  * cos + mv5 * sin;
    m[10] = m[10] * cos + mv9 * sin;
}

fn rotate_y(m: &mut [f32; 16], angle: f32) {
    let (sin, cos) = angle.sin_cos();
    let (mv0, mv4, mv8) = (m[0], m[4], m[8]);

    m[0]  = cos * m[0] + sin * m[2];
    m[4]  = cos * m[4] + sin * m[6];
    m[8]  = cos * m[8] + sin * m[10];

    m[2]  = cos * m[2]  - sin * mv0;
    m[6]  = cos * m[6]  - sin * mv4;
    m[10] = cos * m[10] - sin * mv8;
}

fn rotate_z(m: &mut [f32; 16], angle: f32) {
    let (sin, cos) = angle.sin_cos();
    let (mv0, mv4, mv8) = (m[0], m[4], m[8]);

    m[0] = cos * m[0] - sin * m[1];
    m[4] = cos * m[4] - sin * m[5];
    m[8] = cos * m[8] - sin * m[9];

    m[1] = cos * m[1] + sin * mv0;
    m[5] = cos * m[5] + sin * mv4;
    m[9] = cos * m[9] + sin * mv8;
}