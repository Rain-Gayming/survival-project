use glium;
use glium::Surface;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");

    // window creation
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("voxel game")
        .build(&event_loop);

    // creates a new frame
    let mut frame = display.draw();
    // adds a background
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    // makes the frame visible
    frame.finish().unwrap();

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
