use glium::Surface;
#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    // window creation
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("voxel game")
        .build(&event_loop);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.5, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // vertex shader
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main(){
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    // fragment shader (colour)
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main(){
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    //loads shaders
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    // creates a new frame
    let mut target = display.draw();
    // adds a background
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    //draws the vertex vertex_buffer
    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    // makes the frame visible
    target.finish().unwrap();

    // runs until closed
    event_loop
        .run(move |event, window_target| {
            match event {
                glium::winit::event::Event::WindowEvent { event, .. } => match event {
                    // quits the game when asked
                    glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    _ => (),
                },

                _ => (),
            };
        })
        .unwrap();
}
