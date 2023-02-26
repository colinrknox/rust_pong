use graphics::math::Vec2d;

/// # MotionPhysics
///
/// Uses the basics of kinematics to compute motion
pub struct MotionPhysics {
    position: Vec2d<f64>,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
}

impl MotionPhysics {
    pub fn new(position: Vec2d<f64>) -> MotionPhysics {
        MotionPhysics {
            position,
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    pub fn update(&mut self) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
        self.velocity[0] += self.acceleration[0];
        self.velocity[1] += self.acceleration[1];
    }

    pub fn set_y_position(&mut self, y_coord: f64) {
        self.position[1] = y_coord;
    }

    pub fn get_position(&self) -> Vec2d<f64> {
        self.position
    }

    pub fn set_velocity(&mut self, velocity: Vec2d<f64>) {
        self.velocity = velocity;
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
        let mut motion = MotionPhysics::new([0.0, 0.0]);
        assert_eq!([0.0, 0.0], motion.velocity);
        motion.set_velocity([1.0, 1.0]);
        assert_eq!([1.0, 1.0], motion.velocity);
    }

    #[test]
    fn test_update() {
        let mut motion = MotionPhysics::new([0.0, 0.0]);
        motion.velocity = [2.0, -2.0];
        motion.acceleration = [-0.5, 0.5];
        motion.update();
        assert_eq!([2.0, -2.0], motion.position);
        assert_eq!([1.5, -1.5], motion.velocity);
    }

    #[test]
    fn test_get_position() {
        let motion = MotionPhysics::new([100.0, -39.5]);
        assert_eq!([100.0, -39.5], motion.get_position());
    }
}
