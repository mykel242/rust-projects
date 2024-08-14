use rand::Rng;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

fn main() {
    let mut rng = rand::thread_rng();
    const WIDTH: u32 = 1000;
    const HEIGHT: u32 = 1000;
    const BACK_COLOR: Color = Color::from_rgb(0.2, 0.9, 0.2);

    let window = Window::new_centered("xxx", (WIDTH, HEIGHT)).unwrap();
    let mut w = World {
        _width: WIDTH,
        _height: HEIGHT,
        background: BACK_COLOR,
        shapes: Vec::new(),
    };

    for _ in 0..320 {
        let rx: f32 = rng.gen_range(0.0..WIDTH as f32);
        let ry: f32 = rng.gen_range(0.0..HEIGHT as f32);
        let rp: Point = Point { x: rx, y: ry };
        let mut s = Shape {
            //center: rp.clone(),
            points: Vec::new(),
        };
        s.points.push(rp.clone());
        w.shapes.push(s);
    }

    window.run_loop(MyWindowHandler { world: w });
}

struct MyWindowHandler {
    world: World,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.world.update();
        graphics.clear_screen(self.world.background);

        for s in self.world.shapes.iter() {
            draw_shape(&s, graphics);
        }
        helper.request_redraw();
    }
}

fn draw_shape(s: &Shape, graphics: &mut Graphics2D) {
    let c1 = Color::from_hex_argb(0xFF000000);
    let c2 = Color::from_hex_argb(0x66FFFFFF);

    for p in s.points.iter() {
        graphics.draw_circle((p.x, p.y), 12.0, c2);
    }

    // graphics.draw_circle((s.center.x, s.center.y), 4.0, c1);
}

#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
struct Shape {
    //center: Point,
    points: Vec<Point>,
}

#[derive(Debug)]
struct World {
    _width: u32,
    _height: u32,
    background: Color,
    shapes: Vec<Shape>,
}

impl World {
    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        const STEP: f32 = 4.0;
        // for each shape / add some small random change
        for shape in self.shapes.iter_mut() {
            let delta_x = rng.gen_range(-STEP..STEP);
            let delta_y = rng.gen_range(-STEP..STEP);
            for p in shape.points.iter_mut() {
                p.x += delta_x;
                p.y += delta_y;
            }
        }
    }
}
