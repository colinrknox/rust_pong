pub use super::object::Vec2d;
pub use super::object::Object;
pub use super::object::CollisionWall;

/// # MotionPhysics
///
/// Uses the basics of kinematics to compute motion
#[derive(Debug, PartialEq)]
pub struct MotionPhysics {
    object: Object,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
}

impl MotionPhysics {
    pub fn new(position: Vec2d<f64>, height: f64, width: f64) -> MotionPhysics {
        MotionPhysics {
            object: Object::new(position, height, width),
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    pub fn update_with_bounds(&mut self, height: f64, width: f64) -> Option<CollisionWall> {
        let mut new_position = [
            self.object.get_x() + self.velocity[0],
            self.object.get_y() + self.velocity[1],
        ];
        self.set_velocity([
            self.velocity[0] + self.acceleration[0],
            self.velocity[1] + self.acceleration[1],
        ]);
        self.object
            .update_position_with_bounds(&mut new_position, height, width)
    }

    pub fn set_x(&mut self, x_coord: f64) {
        self.object.set_x(x_coord);
    }

    pub fn set_coords(&mut self, coords: Vec2d<f64>) {
        self.object.set_coords(coords);
    }

    pub fn get_motion_object(&self) -> &Object {
        &self.object
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

    /*
     * MotionPhysics Tests
     */


    /*
     * MotionPhysics Tests
     */
    #[test]
    fn test_update_with_bounds_vertical_wall_bottom() {
        // Arrange
        let mut motion_p = MotionPhysics::new([10.0, 20.0], 5.0, 5.0);
        // Act
        let result = motion_p.update_with_bounds(24.0, 100.0);
        // Assert
        assert_eq!(result, Some(CollisionWall::Vertical));
    }

    #[test]
    fn test_update_with_bounds_vertical_wall_top() {
        let mut motion_p = MotionPhysics::new([5.0, -1.0], 5.0, 5.0);
        let result = motion_p.update_with_bounds(24.0, 100.0);
        assert_eq!(result, Some(CollisionWall::Vertical));
    }

    #[test]
    fn test_update_with_bounds_horizontal_wall_right() {
        let mut motion_p = MotionPhysics::new([10.0, 20.0], 5.0, 5.0);
        let result = motion_p.update_with_bounds(100.0, 14.0);
        assert_eq!(result, Some(CollisionWall::Horizontal));
    }

    #[test]
    fn test_update_with_bounds_horizontal_wall_left() {
        let mut motion_p = MotionPhysics::new([-0.01, 20.0], 5.0, 5.0);
        let result = motion_p.update_with_bounds(100.0, 14.0);
        assert_eq!(result, Some(CollisionWall::Horizontal));
    }

    #[test]
    fn test_update_with_bounds_no_collision() {
        let mut motion_p = MotionPhysics::new([10.0, 20.0], 5.0, 5.0);
        let result = motion_p.update_with_bounds(100.0, 100.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_set_velocity() {
        let mut motion = MotionPhysics::new([0.0, 0.0], 10.0, 10.0);
        assert_eq!([0.0, 0.0], motion.velocity); // Sanity check

        motion.set_velocity([1.0, 1.0]);

        assert_eq!([1.0, 1.0], motion.velocity);
    }

    #[test]
    fn test_motion_physics_set_coords() {
        let mut obj = MotionPhysics::new([0.0, 0.0], 10.0, 10.0);
        obj.set_coords([10.0, -10.0]);
        assert_eq!([10.0, -10.0], [obj.object.get_x(), obj.object.get_y()]);
    }

    #[test]
    fn test_motion_physics_get_motion_object() {
        let obj = MotionPhysics::new([0.0, 10.0], 5.0, 5.0);
        let motion_obj = obj.get_motion_object();
        assert_eq!(obj.object, *motion_obj);
    }

    #[test]
    fn test_motion_physics_set_velocity() {
        let mut obj = MotionPhysics::new([10.0, 10.0], 8.0, 8.0);
        obj.set_velocity([2.0, -1.0]);
        assert_eq!([2.0, -1.0], obj.velocity);
    }

    #[test]
    fn test_motion_physics_set_acceleration() {
        let mut obj = MotionPhysics::new([10.0, 10.0], 3.0, 9.0);
        obj.set_acceleration([1.0, -2.0]);
        assert_eq!([1.0, -2.0], obj.acceleration);
    }
}
