use std::f64::consts::PI;

use wasm_bindgen::prelude::*;
use crate::configuration::CEILING_HEIGHT;

use crate::configuration::CORRIDOR_LENGTH;

#[wasm_bindgen]
pub struct Ball
{
    pub y: f64,
    pub x: f64,
    speed: f64,
    angle: f64
}

impl Ball 
{
    /// angle is in degrees
    pub fn new(start_height: f64, speed: f64, mut angle: f64) -> Ball
    {
        while angle > 360.0
        {
            angle -= 360.0
        }

        while angle < -360.0
        {
            angle += 360.0
        }
        angle = angle*2.0*PI/360.0; // degree -> radian

        Ball {
            y: start_height,
            x: 0.0,
            speed,
            angle
        }
    }

    fn hit_wall(&mut self)
    {
        self.angle = -self.angle;
    }

    pub fn move_tick(&mut self)
    {
        let cos = self.angle.cos();
        let new_x = self.x + cos*self.speed;
        // ! we don't care if it hits the ceiling (for x) only while speed is constant
        let mut speed = self.speed;
        self.x = new_x;
        if new_x >= CORRIDOR_LENGTH
        {
            // the ball has reached the corridor's end. modify the speed to know at which Y it did
            let x_diff = CORRIDOR_LENGTH - self.x;
            speed = x_diff/cos;
            self.x = CORRIDOR_LENGTH;
        }

        let sin = self.angle.sin();
        let mut new_y = self.y + sin*speed;
        if new_y >= CEILING_HEIGHT
        {
            // the ball has reached the ceiling. split the speed in 2 parts, one before hitting and the other after hitting
            // use the second one to calculate final position
            let y_diff = CEILING_HEIGHT - self.y;
            let speed2 = speed-(y_diff/sin);
            self.hit_wall();
            new_y = CEILING_HEIGHT-(sin*speed2).abs();
        } else if new_y <= 0.0
        {
            // the ball has reached the floor. split the speed in 2 parts, one before hitting and the other after hitting
            // use the second one to calculate final position
            let speed2 = speed-(self.y/sin);
            self.hit_wall();
            new_y = -sin*speed2;
        }
        self.y = new_y;
    }

    pub fn has_reached_end(&self) -> bool
    {
        self.x.clone() >= CORRIDOR_LENGTH
    }

    pub fn rethrow(&mut self)
    {
        self.x = 0.0;
    }

    pub fn get_memory(&self) -> BallMemory {
        BallMemory {
            x: self.x,
            y: self.y
        }
    }
}

pub struct BallMemory {
    pub x: f64,
    pub y: f64
}