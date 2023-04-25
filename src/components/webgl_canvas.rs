use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, HtmlCanvasElement, WebGlProgram, WebGlRenderingContext as GL,
    WebGlUniformLocation, console,
};
use yew::prelude::*;

pub struct WebGLState {
    gl: GL,
    program: WebGlProgram,
    time_location: Option<WebGlUniformLocation>,
    mouse_location: Option<WebGlUniformLocation>,
    resolution_location: Option<WebGlUniformLocation>,
}

impl WebGLState {
    fn new(canvas: &HtmlCanvasElement) -> Self {
        let gl = Self::get_webgl_context(canvas).unwrap();
        Self::init_buffers(&gl);
        let program = Self::init_program(&gl);

        let position = gl.get_attrib_location(&program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        let time_location = gl.get_uniform_location(&program, "u_time");
        let mouse_location = gl.get_uniform_location(&program, "u_mouse");
        let dims_location = gl.get_uniform_location(&program, "u_resolution");

        gl.draw_arrays(GL::TRIANGLES, 0, 6);

        Self {
            gl,
            program,
            time_location,
            mouse_location,
            resolution_location: dims_location
        }
    }

    fn init_program(gl: &GL) -> WebGlProgram {
        let vert_code = include_str!("../shaders/basic.vert");
        let frag_code = include_str!("../shaders/basic.frag");

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        shader_program
    }

    fn init_buffers(gl: &GL) {
        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];
        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);
    }

    fn get_webgl_context(canvas: &HtmlCanvasElement) -> Option<GL> {
        let gl: GL = canvas.get_context("webgl").ok()?.unwrap().dyn_into().ok()?;
        Some(gl)
    }
}

pub struct WebGLCanvas {
    node_ref: NodeRef,
}

impl Component for WebGLCanvas {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas class="webgl" ref={self.node_ref.clone()} />
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        web_sys::console::log_1(&"First Render".into());

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);
        let state = Some(WebGLState::new(&canvas));

        let mouse_pos = Rc::new(RefCell::new((0.0, 0.0)));
        let mouse_pos_clone = mouse_pos.clone();
        let cb = Closure::wrap(Box::new(move |event: MouseEvent| {
            let x = event.client_x() as f32;
            let y = event.client_y() as f32;
            *mouse_pos_clone.borrow_mut() = (x, y);
        }) as Box<dyn Fn(_)>);

        canvas.add_event_listener_with_callback("mousemove", cb.as_ref().unchecked_ref()).unwrap();
        cb.forget();
        Self::render_gl(state.unwrap(), mouse_pos);

    }

}

impl WebGLCanvas {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn render_gl(state: WebGLState, mouse_pos: Rc<RefCell<(f32, f32)>>) {
        let mut timestamp = 0.0;

        state.gl.uniform1f(state.time_location.as_ref(), timestamp as f32);
        state.gl.uniform2f(state.mouse_location.as_ref(), mouse_pos.borrow().0, mouse_pos.borrow().1);
        state.gl.uniform2f(state.resolution_location.as_ref(), state.gl.drawing_buffer_width() as f32, state.gl.drawing_buffer_height() as f32);
        state.gl.draw_arrays(GL::TRIANGLES, 0, 6);

        
        let cb = Rc::new(RefCell::new(None));

        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = cb.clone();
            move || {
                timestamp += 20.0;
                state.gl.uniform1f(state.time_location.as_ref(), timestamp as f32);
                state.gl.uniform2f(state.mouse_location.as_ref(), mouse_pos.borrow().0, mouse_pos.borrow().1);
                state.gl.uniform2f(state.resolution_location.as_ref(), state.gl.drawing_buffer_width() as f32, state.gl.drawing_buffer_height() as f32);

                state.gl.draw_arrays(GL::TRIANGLES, 0, 6);

                // console::log_1(&format!("time: {}", timestamp).into());
                // console::log_1(&format!("mouse: {}, {}", mouse_pos.borrow().0, mouse_pos.borrow().1).into());
                // console::log_1(&format!("resolution: {}, {}", state.gl.drawing_buffer_width() as f32, state.gl.drawing_buffer_height() as f32).into());
                Self::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));

        Self::request_animation_frame(cb.borrow().as_ref().unwrap());
    }
}
