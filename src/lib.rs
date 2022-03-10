//wasm-pack build in /www
//npm run start in /www
mod utils;
mod tests;
mod ball;
mod dodger;
mod configuration;

use ball::Ball;
use dodger::Dodger;
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
    balls: Option<Vec<Ball>>,
    dodgers: Option<Vec<Dodger>>,
}

#[wasm_bindgen]
impl World
{
    pub fn new() -> World
    {
        World {
            balls: Some(Vec::new()),
            dodgers: Some(Vec::new())
        }
    }

    pub fn tick(&mut self)
    {
        // BALLS BALLS BALLS BALLS BALLS BALLS BALLS BALLS BALLS
        let mut balls = self.balls.take().unwrap();
        balls.iter_mut()
            .for_each(|i| i.move_tick());
            
        let (mut finalized_balls, mut b): (Vec<Ball>, Vec<Ball>) =
            balls.into_iter().partition(|i| i.has_reached_end());


        // TODO: see if finished_balls have hit any dodgers, instead of throwing them again
        finalized_balls.iter_mut()
            .for_each(|i| i.rethrow());
        
        b.append(&mut finalized_balls);
        self.balls = Some(b);


        // DODGERS DODGERS DODGERS DODGERS DODGERS DODGERS DODGERS
        let mut dodgers = self.dodgers.take().unwrap();
        dodgers.iter_mut()
            .for_each(|i| i.move_tick());
        
        self.dodgers = Some(dodgers);
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
        let balls = match &self.balls {
            Some(x) => x,
            None => panic!("a")
        };
        let mut v = Vec::new();
        for i in balls
        {
            v.push(0.0); // don't question it. It works.
            v.push(i.x);
            v.push(i.y);
        }
        v.as_ptr()
    }

    pub fn get_ball_amount(&self) -> usize {
        let balls = match &self.balls {
            Some(x) => x,
            None => panic!("a")
        };
        balls.len()
    }

    pub fn dodger_positions(&self) -> *const f64 {
        let dodgers = match &self.dodgers {
            Some(x) => x,
            None => panic!("a")
        };
        let mut v = Vec::new();
        for i in dodgers
        {
            v.push(i.height);
            v.push(i.y);
        }
        v.as_ptr()
    }

    pub fn get_dodger_amount(&self) -> usize {
        let dodgers = match &self.dodgers {
            Some(x) => x,
            None => panic!("a")
        };
        dodgers.len()
    }

    /// # Angle is given in degrees
    pub fn add_ball(&mut self, y_pos: f64, speed: f64, angle: f64)
    {
        let mut balls = self.balls.take().unwrap();
        balls.push(Ball::new(y_pos, speed, angle));
        self.balls = Some(balls);
    }

    pub fn add_dodger(&mut self, y_pos: f64, height: f64, max_speed: f64)
    {
        let mut dodgers = self.dodgers.take().unwrap();
        dodgers.push(Dodger::new(y_pos, height, max_speed));
        self.dodgers = Some(dodgers);
    }
}


