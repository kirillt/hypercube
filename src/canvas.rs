use stdweb::web::{
    IEventTarget,
    IHtmlElement,
    IParentNode,
    document,
    window
};

use stdweb::unstable::TryInto;

use webgl_rendering_context::WebGLRenderingContext as gl;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::ResizeEvent;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub fn establish() -> CanvasElement {
    let canvas: CanvasElement = document()
        .query_selector( "#frame" )
        .unwrap().unwrap().try_into().unwrap();

    let context: gl = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    context.clear_color(1.0, 0.0, 0.0, 1.0);
    context.clear(gl::COLOR_BUFFER_BIT);

    window().add_event_listener( enclose!((canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    canvas
}