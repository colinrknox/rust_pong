use graphics::math::Vec2d;

/// # MotionPhysics
///
/// Uses the basics of kinematics to compute motion
pub struct MotionPhysics {
    position: Position,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
}

pub struct Position {
    pub x: f64,
    pub y: f64,
    height: f64,
    width: f64,
}

impl Position {
    pub fn new(top_left: Vec2d, height: f64, width: f64) -> Position {
        Position {
            x: top_left[0],
            y: top_left[1],
            height,
            width,
        }
    }

    pub fn has_collided(&self, other: &Position) -> bool {
        let x2 = self.x + self.width;
        let y2 = self.y + self.height;
        let other_x2 = other.x + other.width;
        let other_y2 = other.y + other.height;

        // Check if collision with left side of other object
        if other.x <= x2
            && x2 <= other_x2
            && ((y2 > other.y && y2 < other_y2) || (self.y > other.y && self.y < other_y2))
        {
            return true;
        }

        // Check if collision with right side of other object
        if other_x2 >= self.x
            && self.x >= other.x
            && ((y2 > other.y && y2 < other_y2) || (self.y > other.y && self.y < other_y2))
        {
            return true;
        }

        return false;
    }
}

impl MotionPhysics {
    pub fn new(position: Vec2d<f64>, height: f64, width: f64) -> MotionPhysics {
        MotionPhysics {
            position: Position::new(position, height, width),
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    pub fn update(&mut self) {
        self.position.x += self.velocity[0];
        self.position.y += self.velocity[1];
        self.velocity[0] += self.acceleration[0];
        self.velocity[1] += self.acceleration[1];
    }

    pub fn set_x(&mut self, x_coord: f64) {
        self.position.x = x_coord;
    }

    pub fn set_y_position(&mut self, y_coord: f64) {
        self.position.y = y_coord;
    }

    pub fn set_position(&mut self, coords: Vec2d<f64>) {
        self.position.x = coords[0];
        self.position.y = coords[1];
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn set_velocity(&mut self, velocity: Vec2d<f64>) {
        self.velocity = velocity;
    }

    pub fn get_velocity(&self) -> Vec2d<f64> {
        self.velocity
    }

    pub fn set_acceleration(&mut self, acceleration: Vec2d<f64>) {
        self.acceleration = acceleration;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_velocity() {
        let mut motion = MotionPhysics::new([0.0, 0.0], 10.0, 10.0);
        assert_eq!([0.0, 0.0], motion.velocity);
        motion.set_velocity([1.0, 1.0]);
        assert_eq!([1.0, 1.0], motion.velocity);
    }

    #[test]
    fn test_update() {
        let mut motion = MotionPhysics::new([0.0, 0.0], 10.0, 10.0);
        motion.velocity = [2.0, -2.0];
        motion.acceleration = [-0.5, 0.5];
        motion.update();
        assert_eq!([2.0, -2.0], motion.position);
        assert_eq!([1.5, -1.5], motion.velocity);
    }

    #[test]
    fn test_get_position() {
        let motion = MotionPhysics::new([100.0, -39.5], 10.0, 10.0);
        assert_eq!([100.0, -39.5], motion.get_position());
    }
}
