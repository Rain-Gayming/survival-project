use glium::{backend::glutin::simple_window_builder::GliumEventLoop, Surface};
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
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main(){
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    "#;

    // fragment shader (colour)
    let fragment_shader_src = r#"
        #version 140
        
        out vec4 color;
        
        in vec3 v_normal;
        uniform vec3 u_light;

        void main(){
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
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

                        // adds a background and depth buffer
                        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                        let x = 0.0;

                        let model = [
                            [0.01, 0.0, 0.0, 0.0],
                            [0.0, 0.01, 0.0, 0.0],
                            [0.0, 0.0,  0.01, 0.0],
                            [0.0, 0.0,  2.25, 1.0f32],
                        ];
                        let perspective = {
                            let (width, height) = target.get_dimensions();
                            let aspect_ratio = height as f32 / width as f32;

                            let fov: f32 = std::f32::consts::PI / 3.0;
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
                        let light = [-1.0, 0.5, 0.9f32];

                        // camera angle i think
                        let view = view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);

                        // rendering options
                        let params = glium::DrawParameters {
                            // depth buffer
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                ..Default::default()
                            },
                            // back face culling
                            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,

                            ..Default::default()
                        };

                        //draws the vertex vertex_buffer
                        target
                            .draw(
                                (&positions, &normals),
                                &indices,
                                &program,
                                &uniform! {model: model, view: view, perspective: perspective, u_light: light},
                                &params,
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

// lol idk

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
