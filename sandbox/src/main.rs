extern crate engine;

use std::{mem, ptr};
use engine::graphics::gl_wrapper::{BufferObject, Vao, VertexAttribute};
use engine::graphics::window::Window;
use engine::types::{GLfloat, GLsizei};

fn main() {
    engine::init();
    let mut window = Window::new(1080, 720, "Hello");
    window.init_gl();

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0,
    ];
    let vao = Vao::new();
    vao.bind();

    let vbo = BufferObject::new(engine::ARRAY_BUFFER, engine::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&vertices);
    let position_attribute = VertexAttribute::new(0,
                                                  3,
                                                  engine::FLOAT,
                                                  engine::FALSE,
                                                  3 * mem::size_of::<GLfloat>() as GLsizei,
                                                  ptr::null());

    position_attribute.enable();


    while !window.should_close() {
        unsafe {
            engine::ClearColor(0.3, 0.5, 0.1, 1.0);
            engine::Clear(engine::COLOR_BUFFER_BIT);
            engine::DrawArrays(engine::TRIANGLES, 0, 3);
        }
        window.update();
    }
}
