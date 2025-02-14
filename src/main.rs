mod world;
use glium::{
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter},
    Surface,
};

use world::voxel::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [u8; 3],
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
            position: [0, 0, 1],
            texture_coords: texture_coords[0],
        },
        Vertex {
            position: [1, 0, 0],
            texture_coords: texture_coords[0],
        },
        Vertex {
            position: [0, 1, 1],
            texture_coords: texture_coords[0],
        },
        Vertex {
            position: [1, 1, 0],
            texture_coords: texture_coords[0],
        },
        Vertex {
            position: [1, 1, 1],
            texture_coords: texture_coords[0],
        },
        Vertex {
            position: [1, 0, 0],
            texture_coords: texture_coords[0],
        },
        Vertex {
            position: [1, 0, 1],
            texture_coords: texture_coords[0],
        },
        Vertex {
            position: [0, 0, 1],
            texture_coords: texture_coords[0],
        },
    ];

    /*
        { 0, 0, 1 }
        { 1, 0, 0 }
        { 0, 1, 1 }
        { 1, 1, 0 }
        { 1, 1, 1 }
        { 1, 0, 0 }
        { 1, 0, 1 }
        { 0, 0, 1 }
    */

    // creates a verte2x buffer and adds teh shape to the display
    let vertex_buffer = glium::VertexBuffer::new(&display, &face).unwrap();
    // creates a blank set of indices
    let cube_indices: [u8; 36] = [
        0, 1, 2, 2, 3, 1, //Bottom
        4, 5, 6, 6, 7, 5, //Front
        8, 9, 10, 10, 11, 9, //Back
        12, 13, 14, 14, 15, 13, //Left
        16, 17, 18, 18, 19, 17, //Right
        20, 21, 22, 22, 23,
        21, /*0, 3, 1,
            1, 3, 2,
            2, 3, 0,
            0, 1, 2*/
    ];

    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &cube_indices,
    )
    .unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec2 texture_coords;
        out vec2 v_texture_coords;

        uniform mat4 matrix;
        uniform mat4 projection;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            // texture_coords / 16 gets the correct sprite.

            mat4 modelview = view * model;
            v_texture_coords = (texture_coords / 16);
            gl_Position = vec4(position, 1.0);
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
                glium::winit::event::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::winit::event::WindowEvent::CloseRequested => {
                            window_target.exit();
                        }
                        // We now need to render everyting in response to a RedrawRequested event due to the animation
                        glium::winit::event::WindowEvent::RedrawRequested => {
                            let mut target = display.draw();
                            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                            let matrix = [
                                [0.1, 0.0, 0.0, 0.0],
                                [0.0, 0.1, 0.0, 0.0],
                                [0.0, 0.0, 0.1, 0.0],
                                [0.0, 0.0, 0.0, 1.0f32],
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

                            let view =
                                view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

                            target
                                .draw(
                                    &vertex_buffer,
                                    &indices,
                                    &program,
                                    &uniform! {view: view, perspective: perspective, light: light, model: matrix},
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
                    }
                }
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

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f[2] - up[2] * f[1],
        up[2] * f[0] - up[0] * f[2],
        up[0] * f[1] - up[1] * f[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f[1] * s_norm[2] - f[2] * s_norm[1],
        f[2] * s_norm[0] - f[0] * s_norm[2],
        f[0] * s_norm[1] - f[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
    ];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}
