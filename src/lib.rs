//wasm-pack build in /www
//npm run start in /www
mod utils;
mod tests;
mod ball;
mod dodger;
mod configuration;
mod ai;

use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

use ball::{Ball, BallMemory};
use dodger::Dodger;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern
{
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct World
{
    balls: Option<Vec<Ball>>,
    dodgers: Option<Vec<Dodger>>,
    memory: VecDeque<Vec<f64>>,
}

#[wasm_bindgen]
impl World
{
    pub fn new() -> World
    {
        let memory_size = 3; // TODO: dont hardcode this
        let memory = World::init_memory(memory_size);
        World {
            balls: Some(Vec::new()),
            dodgers: Some(Vec::new()),
            memory,
        }
    }

    fn init_memory(memory_size: usize) -> VecDeque<Vec<f64>> {
        let mut memory = VecDeque::new();
        for _ in 0..memory_size {
            let mut moment = Vec::new();
            for _ in 0..configuration::DIVISIONS {
                moment.push(0.0);
            }
            memory.push_front(moment);
        }

        memory
    }

    pub fn tick(&mut self)
    {
        // DODGERS DODGERS DODGERS DODGERS DODGERS DODGERS DODGERS
        let mut dodgers = self.dodgers.take().unwrap();
        dodgers.iter_mut()
            .for_each(|i| i.move_tick(&self.memory));

        // BALLS BALLS BALLS BALLS BALLS BALLS BALLS BALLS BALLS
        let mut balls = self.balls.take().unwrap();
        let memories: Vec<BallMemory> = balls.iter_mut()
            .map(|i| {
                i.move_tick();
                i.get_memory()
            }).collect();
        
        self.push_memories(memories);

        let (mut finalized_balls, mut b): (Vec<Ball>, Vec<Ball>) =
            balls.into_iter().partition(|i| i.has_reached_end());


        // TODO: check if the ball hits the dodger midair, instead of seeing if the end position of both collide
        finalized_balls.iter_mut()
            .for_each(
                |i|
                {
                    dodgers.iter_mut().for_each(
                        |j| j.collide_check(i)
                    );
                    i.rethrow();
                }
            );
        
        b.append(&mut finalized_balls);
        self.balls = Some(b);
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

    pub fn ball_positions(&self) -> *const f64
    {
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

    pub fn get_ball_amount(&self) -> usize
    {
        let balls = match &self.balls {
            Some(x) => x,
            None => panic!("a")
        };
        balls.len()
    }

    pub fn dodger_positions(&self) -> *const f64
    {
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

    pub fn get_dodger_amount(&self) -> usize
    {
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

    pub fn get_counters(&mut self) -> *const u32
    {
        let dodgers = match &self.dodgers {
            Some(x) => x,
            None => panic!("a")
        };

        let mut v = Vec::new();
        for i in dodgers
        {
            v.push(i.times_hit);
        }
        
        v.as_ptr()
    }

    /// ticks the memory buffer
    fn push_memories(&mut self, new_memories: Vec<BallMemory>) {
        self.memory.pop_back();
        self.memory.push_front(
            World::moment_from_ballmemories(new_memories)
        );
    }

    /// Divides the height of the corridor into sections
    /// if a section contains a ballmemory its value will be its distance to the dodger.
    /// Where 1 is closest and 0.00(..)1 is farthest.
    /// If there is more than 1 ball in the same section, only the one that is closest appears.
    fn moment_from_ballmemories(ballmemories: Vec<BallMemory>) -> Vec<f64> {
        let mut new_memory = Vec::new();
        for _ in 0..configuration::DIVISIONS {
            new_memory.push(0.0)
        }

        ballmemories.iter()
            .map(|i| (World::get_section(i.y), i.x/configuration::CORRIDOR_LENGTH))
            .for_each(|(position, value)| {
                new_memory[position] = f64::max(value, new_memory[position])
            });

        new_memory
    }

    fn get_section(height: f64) -> usize {
        let mut pos = (height/configuration::DIVISION_HEIGHT).floor() as usize;
        if pos == 200 {
            pos = 199 // because apparently this is not as rare as I thought it would be
        }

        pos
    }
}


