use glium::Surface;
#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

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
        color: [1.0, 0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.5, 0.5],
        color: [0.0, 0.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // vertex shader
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        in vec3 color;
        out vec3 vertex_color;

        uniform mat4 matrix;

        void main(){
            vertex_color = color;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    // fragment shader (colour)
    let fragment_shader_src = r#"
        #version 140
        
        in vec3 vertex_color;
        out vec4 color;

        void main(){
            color = vec4(vertex_color, 1.0);
        }
    "#;

    //loads shaders
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    // runs until closed
    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            match event {
                glium::winit::event::Event::WindowEvent { event, .. } => match event {
                    // quits the game when asked
                    glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),

                    //rendering
                    glium::winit::event::WindowEvent::RedrawRequested => {
                        let x = 0.0;

                        let uniforms = uniform! {
                            matrix: [
                                [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [ x , 0.0, 0.0, 1.0f32],
                            ],
                        };

                        // creates a new frame
                        let mut target = display.draw();
                        // adds a background
                        target.clear_color(0.0, 0.0, 1.0, 1.0);

                        //draws the vertex vertex_buffer
                        target
                            .draw(
                                &vertex_buffer,
                                indices,
                                &program,
                                &uniforms,
                                &Default::default(),
                            )
                            .unwrap();
                        // makes the frame visible
                        target.finish().unwrap();
                    }

                    // when the window's size has changed.
                    glium::winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }

                    _ => (),
                },

                // updates the window
                glium::winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }

                _ => (),
            };
        })
        .unwrap();
}
