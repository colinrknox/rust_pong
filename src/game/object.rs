pub type Vec2d<T> = [T; 2];

#[derive(Debug, PartialEq, Eq)]
pub enum CollisionWall {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq)]
pub struct Object {
    x: f64,
    y: f64,
    height: f64,
    width: f64,
    velocity: Vec2d<f64>,
    acceleration: Vec2d<f64>,
}

impl Object {
    pub fn new(top_left: Vec2d<f64>, height: f64, width: f64) -> Object {
        Object {
            x: top_left[0],
            y: top_left[1],
            height,
            width,
            velocity: [0.0, 0.0],
            acceleration: [0.0, 0.0],
        }
    }

    pub fn update_with_bounds(&mut self, height: f64, width: f64) -> Option<CollisionWall> {
        let mut new_position = [self.x + self.velocity[0], self.y + self.velocity[1]];
        self.velocity[0] += self.acceleration[0];
        self.velocity[1] += self.acceleration[1];

        self.update_position_with_bounds(&mut new_position, height, width)
    }

    pub fn update_position_with_bounds(
        &mut self,
        position: &mut Vec2d<f64>,
        height: f64,
        width: f64,
    ) -> Option<CollisionWall> {
        let mut result = None;
        if position[0] < 0.0 {
            position[0] = 0.0;
            result = Some(CollisionWall::Horizontal);
        }
        if position[0] > width - self.width {
            position[0] = width - self.width;
            result = Some(CollisionWall::Horizontal);
        }
        if position[1] < 0.0 {
            position[1] = 0.0;
            result = Some(CollisionWall::Vertical);
        }
        if position[1] > height - self.height {
            position[1] = height - self.height;
            result = Some(CollisionWall::Vertical);
        }
        self.x = position[0];
        self.y = position[1];
        result
    }

    pub fn has_collided(&self, other: &Object) -> bool {
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

    pub fn get_x(&self) -> f64 {
        self.x
    }
    pub fn get_y(&self) -> f64 {
        self.y
    }
    pub fn get_height(&self) -> f64 {
        self.height
    }
    pub fn get_width(&self) -> f64 {
        self.width
    }
    pub fn get_velocity(&self) -> Vec2d<f64> {
        self.velocity
    }
    // Format for piston window draw rectangle
    pub fn get_size(&self) -> [f64; 4] {
        [self.x, self.y, self.width, self.height]
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    pub fn set_coords(&mut self, coords: Vec2d<f64>) {
        self.x = coords[0];
        self.y = coords[1];
    }
    pub fn set_velocity(&mut self, velocity: Vec2d<f64>) {
        self.velocity = velocity;
    }
}

mod test {
    use super::*;

    #[test]
    fn test_has_collided_left() {
        let obj1 = Object::new([0.0, 0.0], 10.0, 10.0);
        let obj2 = Object::new([5.0, 2.0], 10.0, 10.0);

        let result = obj1.has_collided(&obj2);

        assert_eq!(true, result);
    }

    #[test]
    fn test_has_collided_right() {
        let obj1 = Object::new([15.0, 0.0], 10.0, 10.0);
        let obj2 = Object::new([5.0, 2.0], 10.0, 10.0);

        let result = obj1.has_collided(&obj2);

        assert_eq!(true, result);
    }

    #[test]
    fn test_has_not_collided() {
        let obj1 = Object::new([0.0, 0.0], 10.0, 10.0);
        let obj2 = Object::new([11.0, 2.0], 10.0, 10.0);

        let result = obj1.has_collided(&obj2);

        assert_eq!(false, result);
    }

    #[test]
    fn test_get_x() {
        let obj = Object::new([1.0, 10.0], 5.0, 5.0);
        let result = obj.get_x();
        assert_eq!(1.0, result);
    }

    #[test]
    fn test_get_height() {
        let obj = Object::new([1.0, 10.0], 5.0, 5.0);
        let result = obj.get_height();
        assert_eq!(5.0, result);
    }

    #[test]
    fn test_get_width() {
        let obj = Object::new([1.0, 10.0], 5.0, 5.0);
        let result = obj.get_width();
        assert_eq!(5.0, result);
    }

    #[test]
    fn test_get_size() {
        let obj = Object::new([1.0, 10.0], 5.0, 5.0);
        let result = obj.get_size();
        assert_eq!([1.0, 10.0, 5.0, 5.0], result);
    }

    #[test]
    fn test_update_with_bounds_vertical_wall_bottom() {
        // Arrange
        let mut motion_p = Object::new([10.0, 20.0], 5.0, 5.0);
        // Act
        let result = motion_p.update_with_bounds(24.0, 100.0);
        // Assert
        assert_eq!(result, Some(CollisionWall::Vertical));
    }

    #[test]
    fn test_update_with_bounds_vertical_wall_top() {
        let mut motion_p = Object::new([5.0, -1.0], 5.0, 5.0);
        let result = motion_p.update_with_bounds(24.0, 100.0);
        assert_eq!(result, Some(CollisionWall::Vertical));
    }

    #[test]
    fn test_update_with_bounds_horizontal_wall_right() {
        let mut motion_p = Object::new([10.0, 20.0], 5.0, 5.0);
        let result = motion_p.update_with_bounds(100.0, 14.0);
        assert_eq!(result, Some(CollisionWall::Horizontal));
    }

    #[test]
    fn test_update_with_bounds_horizontal_wall_left() {
        let mut motion_p = Object::new([-0.01, 20.0], 5.0, 5.0);
        let result = motion_p.update_with_bounds(100.0, 14.0);
        assert_eq!(result, Some(CollisionWall::Horizontal));
    }

    #[test]
    fn test_update_with_bounds_no_collision() {
        let mut motion_p = Object::new([10.0, 20.0], 5.0, 5.0);
        let result = motion_p.update_with_bounds(100.0, 100.0);
        assert_eq!(result, None);
    }
}
