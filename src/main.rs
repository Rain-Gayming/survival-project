use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    colour: [f32; 3],
}
implement_vertex!(Vertex, position, colour);

#[macro_use]
extern crate glium;
fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #3")
        .build(&event_loop);

    let mut frame = display.draw();
    frame.clear_color(1.0, 1.0, 1.0, 1.0);
    frame.finish().unwrap();

    // vertices for a triangle
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        colour: [0.0, 0.0, 1.0],
    };
    let vertex2 = Vertex {
        position: [0.5, 0.5],
        colour: [0.0, 1.0, 0.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.5],
        colour: [0.0, 0.0, 0.0],
    };
    // adds the vertices to a shape array
    let shape = vec![vertex1, vertex2, vertex3];

    // creates a vertex buffer and adds teh shape to the display
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // creates a blank set of indices
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // creates a vertex shader
    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        uniform float x_off; 
        uniform mat4 matrix;

        in vec3 colour;
        out vec3 vertex_colour;
        void main() {
            vertex_colour = colour; 
            vec2 pos = position;
            pos.x += x_off;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    // adds colour to the shape
    let fragment_shader_src = r#"

    #version 140
        
        in vec3 vertex_colour;
        out vec4 colour;

        void main() {
            colour = vec4(vertex_colour, 1.0);
         }
    "#;

    // gives glium the shaders made
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = 0.0;

    // draw the triangle here
    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| {
            match ev {
                glium::winit::event::Event::WindowEvent { event, .. } => match event {
                    glium::winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    // We now need to render everyting in response to a RedrawRequested event due to the animation
                    glium::winit::event::WindowEvent::RedrawRequested => {
                        // we update `t`
                        t += 0.02;
                        let x = t.sin() * 0.5;

                        let mut target = display.draw();
                        target.clear_color(0.0, 0.0, 1.0, 1.0);

                        let uniforms = uniform! {
                            matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [  x, 0.0, 0.0, 1.0f32],
                        ]
                        };

                        target
                            .draw(
                                &vertex_buffer,
                                &indices,
                                &program,
                                &uniforms,
                                &Default::default(),
                            )
                            .unwrap();
                        target.finish().unwrap();
                    }
                    // Because glium doesn't know about windows we need to resize the display
                    // when the window's size has changed.
                    glium::winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => (),
                },
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                glium::winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .unwrap();
}
