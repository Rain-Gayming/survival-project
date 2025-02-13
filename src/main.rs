mod world;
use glium::{
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    Surface,
};

use world::voxel::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [u8; 2],
    texture_coords: [i8; 2],
}
implement_vertex!(Vertex, position, texture_coords);

#[macro_use]
extern crate glium;
fn main() {
    // update loop
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    // creates the window
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Voxel Engine")
        .build(&event_loop);

    // draws the background
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

    // new voxel
    let new_voxel = Voxel {
        block_type: BlockType::Dirt,
        texture_position: [1, 16],
    };

    let texture_coords = [
        //top left
        [
            new_voxel.texture_position[0] - 1,
            new_voxel.texture_position[1],
        ],
        //bottom left
        [
            new_voxel.texture_position[0] - 1,
            new_voxel.texture_position[1] - 1,
        ],
        //top right
        [new_voxel.texture_position[0], new_voxel.texture_position[1]],
        //bottom right
        [
            new_voxel.texture_position[0],
            new_voxel.texture_position[1] - 1,
        ],
    ];

    let face = vec![
        // top left
        Vertex {
            position: [0, 1],
            texture_coords: texture_coords[0],
        },
        // bottom left
        Vertex {
            position: [0, 0],
            texture_coords: texture_coords[1],
        },
        // top right
        Vertex {
            position: [1, 1],
            texture_coords: texture_coords[2],
        },
        // bottom left
        Vertex {
            position: [0, 0],
            texture_coords: texture_coords[1],
        },
        // top right
        Vertex {
            position: [1, 1],
            texture_coords: texture_coords[2],
        },
        // bottom right
        Vertex {
            position: [1, 0],
            texture_coords: texture_coords[3],
        },
    ];

    // creates a vertex buffer and adds teh shape to the display
    let vertex_buffer = glium::VertexBuffer::new(&display, &face).unwrap();
    // creates a blank set of indices
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 texture_coords;
        out vec2 v_texture_coords;

        uniform mat4 matrix;

        void main() {
            // texture_coords / 16 gets the correct sprite.

            v_texture_coords = (texture_coords / 16);
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        in vec2 v_texture_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_texture_coords);
        }
    "#;

    // gives glium the shaders made
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };
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
                        let mut target = display.draw();
                        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                        let matrix = [
                            [0.5, 0.0, 0.0, 0.0],
                            [0.0, 0.5, 0.0, 0.0],
                            [0.0, 0.0, 0.5, 0.0],
                            [0.0, 0.0, -0.5, 1.0f32],
                        ];

                        let perspective = {
                            let (width, height) = target.get_dimensions();
                            let aspect_ratio = height as f32 / width as f32;

                            let fov: f32 = 3.141592 / 3.0;
                            let zfar = 1024.0;
                            let znear = 0.1;

                            let f = 1.0 / (fov / 2.0).tan();

                            [
                                [f * aspect_ratio, 0.0, 0.0, 0.0],
                                [0.0, f, 0.0, 0.0],
                                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
                            ]
                        };

                        let light = [-1.0, 0.4, 0.9f32];

                        let params = glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        };

                        target
                            .draw(
                                &vertex_buffer,
                                &indices,
                                &program,
                                &uniform! {
                                    matrix: matrix,
                                    perspective: perspective,
                                    u_light: light,
                                    tex: glium::uniforms::Sampler(&texture, behavior),
                                },
                                &params,
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
