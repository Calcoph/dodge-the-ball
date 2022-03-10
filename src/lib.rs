//wasm-pack build in /www
//npm run start in /www
mod utils;
mod tests;
mod ball;
mod dodger;

use ball::Ball;
use dodger::Dodger;
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
    balls: Option<Vec<Ball>>,
    dodgers: Option<Vec<Dodger>>,
}

#[wasm_bindgen]
impl World
{
    pub fn new() -> World
    {
        World {
            balls: Some(vec![
                Ball::new(50.0, 10.0, 45.0),
                Ball::new(50.0, 10.0, 60.0),
                Ball::new(50.0, 10.0, 80.0),
                Ball::new(50.0, 10.0, 30.0),
                Ball::new(50.0, 10.0, 15.0),
                ]),
            dodgers: Some(vec![
                Dodger::new(50.0, 1.0),
                Dodger::new(50.0, -1.0),
            ])
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
}


