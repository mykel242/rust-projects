use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

fn main() {
    let window = Window::new_centered("xxx", (600, 600)).unwrap();
    window.run_loop(MyWindowHandler {});
}

struct MyWindowHandler {}
impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.2, 0.9, 0.2));
        let c = Color::from_hex_argb(0x88000000);
        graphics.draw_circle((300.0, 300.0), 125.0, c.clone());
        graphics.draw_circle((425.0, 175.0), 100.0, c.clone());
        graphics.draw_circle((175.0, 175.0), 100.0, c.clone());
        helper.request_redraw();
    }
}
