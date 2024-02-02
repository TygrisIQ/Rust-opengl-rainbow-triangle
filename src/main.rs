extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::{log_errors, Context};
use std::ffi::{c_void, CString};
use std::{mem, ptr};

const VERTEX_SRC: &str = r#"

    #version 330 core
    layout (location =1 )in vec3 incolor;
    layout (location = 0) in vec3 pos;
    out vec4 outcolor;
    void main(){ 
    gl_Position = vec4(pos, 1.0f);
    outcolor = vec4(incolor, 1.0f);
     }
"#;

const FRAGMENT_SRC: &str = r#"

    #version 330 core
    out vec4 fcolor;

    in vec4 outcolor;
    void main(){
    fcolor = outcolor;
    }
"#;

#[allow(non_snake_case)]
fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(640, 480, "SE", glfw::WindowMode::Windowed)
        .expect("FAILED WINDOW CREATION");

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    //window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let (shaderProgram, VAO) = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let cstrv = CString::new(VERTEX_SRC.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &cstrv.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let cstrf = CString::new(FRAGMENT_SRC.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &cstrf.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let shaderProgram = gl::CreateProgram();
        gl::AttachShader(shaderProgram, vertex_shader);
        gl::AttachShader(shaderProgram, fragment_shader);
        gl::LinkProgram(shaderProgram);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let verticies: [f32; 18] = [
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, //left side
            0.0, 0.5, 0.0, 0.0, 1.0, 0.0, //top side
            0.5, -0.5, 0.0, 0.0, 1.0, 1.0, //right side...
        ];

        let (mut VAO, mut VBO) = (0, 0);

        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::BindVertexArray(VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (verticies.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &verticies[0] as *const f32 as *const _,
            gl::STATIC_DRAW,
        );
        let stride = (mem::size_of::<GLfloat>() * 6) as GLsizei;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);

        (shaderProgram, VAO)
    };
    while !window.should_close() {
        glfw.poll_events();
        window.swap_buffers();
        handle_events(&mut window, &events);
        unsafe {
            dbg!(glfw::get_error());
            dbg!(gl::GetError());
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(VAO);
            gl::UseProgram(shaderProgram);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn handle_events(window: &mut glfw::Window, events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                window.set_should_close(true);
            }
            glfw::WindowEvent::FramebufferSize(w, h) => unsafe {
                gl::Viewport(0, 0, w, h);
            },
            _ => {}
        }
    }
}
