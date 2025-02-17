use glium::Surface;
#[macro_use]
extern crate glium;

mod teapot;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    // window creation
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("voxel game")
        .build(&event_loop);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    // vertex shader
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        uniform mat4 matrix;

        void main(){
             gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    // fragment shader (colour)
    let fragment_shader_src = r#"
        #version 140
        
        out vec4 color;

        void main(){
            color = vec4(1.0, 1.0, 0.0, 1.0);
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
                        // creates a new frame
                        let mut target = display.draw();
                        // adds a background
                        target.clear_color(0.0, 0.0, 1.0, 1.0);

                        let x = 0.0;

                        let matrix = [
                            [0.01, 0.0, 0.0, 0.0],
                            [0.0, 0.01, 0.0, 0.0],
                            [0.0, 0.0, 0.01, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ];
                        //draws the vertex vertex_buffer
                        target
                            .draw(
                                (&positions, &normals),
                                &indices,
                                &program,
                                &uniform! {matrix: matrix},
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
