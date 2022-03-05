//wasm-pack build in /www
//npm run start in /www
mod utils;
mod tests;
pub mod ball;

use ball::Ball;
use ball::configuration;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct World {
    balls: Vec<Ball>,
}

#[wasm_bindgen]
impl World
{
    pub fn new() -> World
    {
        World{
            balls: vec![Ball::new(50.0, 10.0, 45.0)],
        }
    }

    pub fn tick(&mut self)
    {
        for i in &mut self.balls
        {
            i.move_tick();
        }
    }

    pub fn ceiling_height(&self) -> f64
    {
        configuration::CEILING_HEIGHT
    }

    pub fn corridor_length(&self) -> f64
    {
        configuration::CORRIDOR_LENGTH
    }

    pub fn ball_positions(&self) -> *const f64 {
        let mut v = Vec::new();
        for i in &self.balls
        {
            v.push(i.x);
            v.push(i.y);
        }

        v.as_ptr()
    }

    pub fn get_ball_amount(&self) -> usize {
        self.balls.len()
    }
}


