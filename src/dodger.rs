use crate::ball::configuration::CEILING_HEIGHT;

pub struct Dodger {
    pub y: f64,
    speed: f64
}

impl Dodger
{
    /// angle is in degrees
    pub fn new(y_pos: f64, speed: f64) -> Dodger
    {
        Dodger {
            y: y_pos,
            speed,
        }
    }

    pub fn change_speed(&self) {

    }

    pub fn move_tick(&mut self) {
        let mut new_y = self.y + self.speed;
        if new_y >= CEILING_HEIGHT
        {
            new_y = CEILING_HEIGHT
        } else if new_y <= 0.0
        {
            new_y = 0.0
        }
        self.y = new_y;
    }
}