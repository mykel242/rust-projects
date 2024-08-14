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

    // let z = Shape {
    //     points: vec![
    //         Point { x: 0.0, y: 0.0 },
    //         Point { x: 100.0, y: 0.0 },
    //         Point { x: 100.0, y: 100.0 },
    //         Point { x: 0.0, y: 100.0 },
    //     ],
    //     x_speed: 0.5,
    //     y_speed: 5.0,
    // };
    // w.shapes.push(z);

    for _ in 0..999 {
        let rx: f32 = rng.gen_range(0.0..WIDTH as f32);
        let ry: f32 = rng.gen_range(0.0..HEIGHT as f32);
        let rp: Point = Point { x: rx, y: ry };
        let mut s = Shape {
            points: Vec::new(),
            x_speed: rng.gen_range(-2.0..6.0),
            y_speed: rng.gen_range(-2.0..6.0),
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
    let c2 = Color::from_hex_argb(0x66FFFFFF);
    for p in s.points.iter() {
        graphics.draw_circle((p.x, p.y), 12.0, c2);
    }
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
    x_speed: f32,
    y_speed: f32,
}

impl Shape {
    fn update(&mut self) {
        //TODO: pass a reference to the bounding rectangle of "the world"
        // to get rid of the hardcoded 1000.0
        for p in self.points.iter_mut() {
            p.x += self.x_speed;
            p.y += self.y_speed;

            if p.x > 1000.0 || p.x < 0.0 {
                self.x_speed *= -1.0;
            }

            if p.y > 1000.0 || p.y < 0.0 {
                self.y_speed *= -1.0;
            }
        }
    }
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
        for shape in self.shapes.iter_mut() {
            shape.update();
        }
    }
}
