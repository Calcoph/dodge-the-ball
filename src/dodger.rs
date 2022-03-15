use std::collections::VecDeque;

use crate::configuration::CEILING_HEIGHT;
use crate::ball::{Ball, BallMemory};
use crate::ai::Brain;
use js_sys;
use wasm_bindgen::memory;

pub struct Dodger {
    pub y: f64,
    pub height: f64,
    pub times_hit: u32,
    speed: f64,
    max_speed: f64,
    brain: Brain
}

impl Dodger
{
    /// angle is in degrees
    pub fn new(y_pos: f64, height: f64, max_speed: f64) -> Dodger
    {
        Dodger {
            y: y_pos,
            height: height,
            speed: max_speed, // TODO: make speed start at 0, it's max_speed because ther is no AI yet
            max_speed,
            times_hit: 0,
            brain: Brain::new_random(vec![3,4,5,1,6,1])// Brain{inputs: 0, layers: vec![]}// Brain::new_random(vec![3,4,5,1,6,1]) // TODO dont hardcode the amount of neurons per layer
        }
    }

    pub fn change_speed(&mut self) {
        let inputs = vec![ // TODO dont hardcode the inputs
            js_sys::Math::random(),
            js_sys::Math::random(),
            js_sys::Math::random()
        ];
        let speed = self.brain.get_output(inputs);
        if speed.abs() <= self.max_speed {
            self.speed = speed;
        }
    }

    pub fn move_tick(&mut self, memory: &VecDeque<Vec<f64>>) {
        self.change_speed();

        let mut new_y = self.y + self.speed;
        if new_y+self.height >= CEILING_HEIGHT
        {
            new_y = CEILING_HEIGHT-self.height
        } else if new_y <= 0.0
        {
            new_y = 0.0
        }
        self.y = new_y;
    }

    pub fn collide_check(&mut self, ball: &Ball)
    {
        if ball.y >= self.y && ball.y <= self.y+self.height {
            self.times_hit += 1;
        }
    }
}
