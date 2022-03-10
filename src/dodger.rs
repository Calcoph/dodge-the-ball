use crate::configuration::CEILING_HEIGHT;

pub struct Dodger {
    pub y: f64,
    pub height: f64,
    speed: f64,
    max_speed: f64,
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
            max_speed
        }
    }

    pub fn change_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn move_tick(&mut self) {
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
}