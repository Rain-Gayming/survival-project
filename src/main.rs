use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

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

    // gets the texture atlas
    let image = image::load(
        std::io::Cursor::new(&include_bytes!("../assets/textures/texture-atlas.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();

    // gets its coordinates
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    // creates a texture from the image
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let shape = vec![
        // top left
        Vertex {
            position: [0.0, 1.0],
            tex_coords: [0.0, 16.0],
        },
        // bottom left
        Vertex {
            position: [0.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        // top right
        Vertex {
            position: [1.0, 1.0],
            tex_coords: [16.0, 16.0],
        },
        // bottom left
        Vertex {
            position: [0.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        // top right
        Vertex {
            position: [1.0, 1.0],
            tex_coords: [16.0, 16.0],
        },
        // bottom right
        Vertex {
            position: [1.0, 0.0],
            tex_coords: [16.0, 0.0],
        },
    ];

    // creates a vertex buffer and adds teh shape to the display
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // creates a blank set of indices
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords / 16;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
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
                                [0.5, 0.0, 0.0, 0.0],
                                [0.0, 0.5, 0.0, 0.0],
                                [0.0, 0.0, 0.5, 0.0],
                                [  x, 0.0, 0.0, 1.0f32],
                            ],
                            tex: &texture,
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
