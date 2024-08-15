#![allow(dead_code)]

use glam::f32::Vec2;
use rand::Rng;
use speedy2d::color::Color;
use speedy2d::dimen::UVec2;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::Window;

fn main() {
    let mut rng = rand::thread_rng();
    const WIDTH: u32 = 1000;
    const HEIGHT: u32 = 1000;
    const BACK_COLOR: Color = Color::from_rgb(0.2, 0.9, 0.2);
    const NUM_SHAPES: u32 = 1000;

    let window = Window::new_centered("xp-noc ex 1.2", (WIDTH, HEIGHT)).unwrap();
    let mut w = World {
        _width: WIDTH,
        _height: HEIGHT,
        background: BACK_COLOR,
        shapes: Vec::new(),
    };

    for _ in 0..NUM_SHAPES {
        let mut s = Shape {
            points: vec![],
            speed: Vec2 {
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0),
            },
            bounds: (Vec2 { x: 0.0, y: 0.0 }, Vec2 { x: 0.0, y: 0.0 }),
        };

        for _ in 0..rng.gen_range(1..10) {
            let p = Vec2 {
                x: rng.gen_range(400.0..600.0),
                y: rng.gen_range(400.0..600.0),
            };
            s.add_point(p);
        }

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

    /// Invoked when the window is resized.

    fn on_resize(&mut self, helper: &mut WindowHelper, size_pixels: UVec2) {
        println!("Resized: {:#?}", size_pixels);
        self.world._width = size_pixels.x;
        self.world._height = size_pixels.y;
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
struct Shape {
    points: Vec<Vec2>,
    speed: Vec2,
    bounds: (Vec2, Vec2),
}

impl Shape {
    fn add_point(&mut self, p: Vec2) {
        self.points.push(p);
    }

    fn update_position(&mut self) {
        for p in self.points.iter_mut() {
            p.x += self.speed.x;
            p.y += self.speed.y;
        }
    }

    fn update_bounds(&mut self) {
        // defines a bounding rectangle for an arbitrary set of points.
        // cheap but low-fidelity collision detection.
        let mut left = f32::MAX;
        let mut right = f32::MIN;
        let mut top = f32::MAX;
        let mut bottom = f32::MIN;

        for p in self.points.iter_mut() {
            if p.x < left {
                left = p.x;
            }
            if p.x > right {
                right = p.x;
            }
            if p.y < top {
                top = p.y;
            }
            if p.y > bottom {
                bottom = p.y;
            }
        }
        self.bounds.0.x = left;
        self.bounds.0.y = top;
        self.bounds.1.x = right;
        self.bounds.1.y = bottom;
    }

    fn update_speed(&mut self, min_x: f32, max_x: f32, min_y: f32, max_y: f32) {
        let old_speed = self.speed.clone();
        let mut new_speed = self.speed.clone();

        if self.bounds.0.x <= min_x && old_speed.x < 0.0 {
            new_speed.x = old_speed.x.abs();
        }

        if self.bounds.1.x >= max_x && old_speed.x > 0.0 {
            new_speed.x = old_speed.x.abs() * -1.0;
        }

        if self.bounds.0.y <= min_y && old_speed.y < 0.0 {
            new_speed.y = old_speed.y.abs();
        }

        if self.bounds.1.y >= max_y && old_speed.y > 0.0 {
            new_speed.y = old_speed.y.abs() * -1.0;
        }

        if old_speed != new_speed {
            // println!(">>> Change the speed! <<< ");
            self.speed = new_speed;
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
            shape.update_position();
            shape.update_bounds();
            shape.update_speed(0.0, self._width as f32, 0.0, self._height as f32);
        }
    }
}
