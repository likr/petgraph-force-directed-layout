extern crate petgraph;
extern crate fdlayout;
extern crate gleam;
extern crate glutin;
extern crate libc;

use std::mem;
use std::ptr;
use gleam::gl;
use petgraph::graph::Graph;
use fdlayout::graph::{Node, Edge};
use fdlayout::layout::layout;

fn main() {
    let mut graph = Graph::<Node, Edge>::new();
    let u1 = graph.add_node(Node {id: String::from("1"), degree: 0});
    let u2 = graph.add_node(Node {id: String::from("2"), degree: 0});
    let u3 = graph.add_node(Node {id: String::from("3"), degree: 0});
    let u4 = graph.add_node(Node {id: String::from("4"), degree: 0});
    let u5 = graph.add_node(Node {id: String::from("5"), degree: 0});
    graph.add_edge(u1, u2, Edge {});
    graph.add_edge(u1, u3, Edge {});
    graph.add_edge(u2, u3, Edge {});
    graph.add_edge(u4, u5, Edge {});
    let positions = layout(&graph);

    let mut vertex_data = Vec::with_capacity(graph.node_count() * 5);
    for i in 0..graph.node_count() {
        vertex_data.push(positions[i].x / 30.0);
        vertex_data.push(positions[i].y / 30.0);
        vertex_data.push(1.0);
        vertex_data.push(0.0);
        vertex_data.push(0.0);
    }

    let window = glutin::Window::new().unwrap();

    unsafe {
        let _ = window.make_current();
    }

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        let vs = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
        gl::CompileShader(vs);

        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
        gl::CompileShader(fs);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        gl::UseProgram(program);

        let mut vb = mem::uninitialized();
        gl::GenBuffers(1, &mut vb);
        gl::BindBuffer(gl::ARRAY_BUFFER, vb);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertex_data.len() * mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                       vertex_data.as_ptr() as *const _, gl::STATIC_DRAW);

        if gl::BindVertexArray::is_loaded() {
            let mut vao = mem::uninitialized();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        let f32_size = mem::size_of::<f32>();
        let pos_attrib = gl::GetAttribLocation(program, b"position\0".as_ptr() as *const _);
        let color_attrib = gl::GetAttribLocation(program, b"color\0".as_ptr() as *const _);
        gl::VertexAttribPointer(pos_attrib as gl::types::GLuint, 2, gl::FLOAT, 0,
                                5 * f32_size as gl::types::GLsizei,
                                ptr::null());
        gl::VertexAttribPointer(color_attrib as gl::types::GLuint, 3, gl::FLOAT, 0,
                                5 * f32_size as gl::types::GLsizei,
                                (2 * f32_size) as *const () as *const _);
        gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
        gl::EnableVertexAttribArray(color_attrib as gl::types::GLuint);
        gl::PointSize(10.0);
    }

    for event in window.wait_events() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::POINTS, 0, graph.node_count() as gl::types::GLsizei);
        }

        let _ = window.swap_buffers();

        match event {
            glutin::Event::Closed => break,
            _ => (),
        }
    }
}

const VS_SRC: &'static [u8] = b"
#version 100
precision mediump float;
attribute vec2 position;
attribute vec3 color;
varying vec3 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

const FS_SRC: &'static [u8] = b"
#version 100
precision mediump float;
varying vec3 v_color;
void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";
