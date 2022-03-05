use configuration::WALL_HEIGHT;

impl Ball 
{
    /// angle is in degrees
    pub fn new(y_pos: u32, speed: f64, angle: f64) -> Ball
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
            y_pos,
            speed,
            angle
        }
    }

    pub fn hit_wall(&mut self)
    {
        self.angle = -self.angle;
    }
}