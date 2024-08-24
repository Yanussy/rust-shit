use ggez::conf::WindowMode;
use ggez::event;
use ggez::glam::*;
use ggez::graphics::DrawParam;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use rand::Rng;

struct MainState {
    v: Vec<(i32, i32)>,
}
//
impl MainState {
    fn new() -> GameResult<MainState> {
        let mut v = Vec::new();
        for _r in 0..100 {
            v.push((
                rand::thread_rng().gen_range(0..600),
                rand::thread_rng().gen_range(0..600),
            ))
        }

        let s = MainState { v };
        Ok(s)
    }
}
pub fn dist(x: f64, x1: f64, y: f64, y1: f64) -> f64 {
    return ((x - x1) * (x - x1) + (y - y1) * (y - y1)).sqrt();
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let mut color = 255;

        for (x, y) in &self.v {
            for (x1, y1) in &self.v {
                if x1 != x && y1 != y && dist(*x as f64, *x1 as f64, *y as f64, *y1 as f64) <= 100.0
                {
                    color -= 1;
                    let _line = graphics::Mesh::new_line(
                        ctx,
                        &[
                            Vec2::new(*x as f32, *y as f32),
                            Vec2::new(*x1 as f32, *y1 as f32),
                        ],
                        1.0,
                        Color::from_rgb((color as u8) / 2, (color as u8) / 1 / 5, color as u8),
                    );
                    canvas.draw(&_line.unwrap(), DrawParam::default());
                }
            }
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}
pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(WindowMode::default().dimensions(600.0, 600.0));
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
