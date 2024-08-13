use pixels::{Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::keyboard::KeyCode;
use tao::menu::{MenuBar, MenuItem};
use tao::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;
struct World {
    // box_x: i16,
    // box_y: i16,
    // velocity_x: i16,
    // velocity_y: i16,
}

fn main() {
    let event_loop = EventLoop::new();
    let window = {
        let mut file_menu = MenuBar::new();
        file_menu.add_native_item(MenuItem::Quit);

        let mut menu = MenuBar::new();
        menu.add_submenu("File", true, file_menu);

        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels/Tao")
            .with_menu(menu)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                // Close events
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::Escape,
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }

                // Resize the window
                WindowEvent::Resized(size) => {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        println!("pixels.resize_surface: {}", err);
                        *control_flow = ControlFlow::Exit;
                    }
                }

                _ => {}
            },

            // Update internal state and request a redraw
            Event::MainEventsCleared => {
                world.update();
                window.request_redraw();
            }

            // Draw the current frame
            Event::RedrawRequested(_) => {
                world.draw(pixels.frame_mut());
                if let Err(err) = pixels.render() {
                    println!("pixels.render: {}", err);
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {}
        }
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            // box_x: 24,
            // box_y: 16,
            // velocity_x: 1,
            // velocity_y: 1,
        }
    }

    fn update(&mut self) {}

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // let x = (i % WIDTH as usize) as i16;
            // let y = (i / WIDTH as usize) as i16;

            // let inside_the_box = x >= self.box_x
            //     && x < self.box_x + BOX_SIZE
            //     && y >= self.box_y
            //     && y < self.box_y + BOX_SIZE;

            // let rgba = if inside_the_box {
            //     [0x5e, 0x48, 0xe8, 0xff]
            // } else {
            //     [0x48, 0xb2, 0xe8, 0xff]
            // };

            let rgba = [0x00, 0xff, 0x00, 0xff];
            pixel.copy_from_slice(&rgba);
        }
    }
}
