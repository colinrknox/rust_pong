use graphics::math::Vec2d;
use kinematics::MotionPhysics;
use rand::prelude::*;

pub mod kinematics;

// Constants for object properties like the paddles, ball, game size etc.
pub const PADDLE_VELOCITY: f64 = 2.0;
pub const BALL_VELOCITY: f64 = 2.0;
pub const PADDLE_HEIGHT: f64 = 38.0;
pub const PADDLE_WIDTH: f64 = 8.0;
pub const BALL_SIZE: f64 = 7.0;
pub const GOAL_BUFFER: f64 = 40.0;
pub const PONG_WIDTH: f64 = 1024.0;
pub const PONG_HEIGHT: f64 = 512.0;

pub const GAME_OBJECT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub trait GameObject {
    fn get_size(&self) -> [f64; 4];
    fn get_color(&self) -> [f32; 4];
    fn update(&mut self);
}

pub enum PaddleType {
    Left,
    Right,
}

pub struct Pong {
    height: f64,
    width: f64,
    score_player_one: usize,
    score_player_two: usize,
    paddle_left: Paddle,
    paddle_right: Paddle,
    ball: Ball,
}

pub struct Ball {
    motion: MotionPhysics,
    color: [f32; 4],
}

pub struct Paddle {
    motion: MotionPhysics,
    color: [f32; 4],
}

impl Pong {
    pub fn new() -> Pong {
        Pong {
            height: PONG_HEIGHT,
            width: PONG_WIDTH,
            score_player_one: 0,
            score_player_two: 0,
            paddle_left: Paddle::new(PaddleType::Left),
            paddle_right: Paddle::new(PaddleType::Right),
            ball: Ball::new(),
        }
    }

    pub fn get_player_one_score(&self) -> usize {
        self.score_player_one
    }

    pub fn get_player_two_score(&self) -> usize {
        self.score_player_two
    }

    pub fn update(&mut self) {
        self.paddle_left.update();
        self.paddle_right.update();
        self.ball.update();
        self.resolve_collisions();
        if self.ball.motion.get_motion_object().x < 1.0 {
            self.score_player_two += 1;
            self.reset_to_initial_conditions();
        }
        if self.ball.motion.get_motion_object().x > PONG_WIDTH - BALL_SIZE - 1.0 {
            self.score_player_one += 1;
            self.reset_to_initial_conditions();
        }
    }

    pub fn update_left_paddle_velocity(&mut self, velocity: Vec2d<f64>) {
        self.paddle_left.motion.set_velocity(velocity);
    }

    pub fn update_right_paddle_velocity(&mut self, velocity: Vec2d<f64>) {
        self.paddle_right.motion.set_velocity(velocity);
    }

    pub fn get_entities(&self) -> Vec<Box<&dyn GameObject>> {
        vec![
            Box::new(&self.paddle_right),
            Box::new(&self.paddle_left),
            Box::new(&self.ball),
        ]
    }

    fn resolve_collisions(&mut self) {
        if self
            .ball
            .motion
            .get_motion_object()
            .has_collided(&self.paddle_right.motion.get_motion_object())
            || self
                .ball
                .motion
                .get_motion_object()
                .has_collided(&self.paddle_left.motion.get_motion_object())
        {
            let mut rng = thread_rng();
            let range: f64 = rng.gen_range(-BALL_VELOCITY..BALL_VELOCITY);
            let sign = if self.ball.motion.get_velocity()[0] < 0.0 {
                1.0
            } else {
                -1.0
            };
            self.ball
                .motion
                .set_velocity([(7.0 - range.powi(2)).sqrt() * sign, range]);
        }
    }

    fn reset_to_initial_conditions(&mut self) {
        self.ball = Ball::new();
        self.paddle_right = Paddle::new(PaddleType::Right);
        self.paddle_left = Paddle::new(PaddleType::Left);
    }
}

impl Paddle {
    pub fn new(p_type: PaddleType) -> Self {
        match p_type {
            PaddleType::Left => Paddle {
                motion: MotionPhysics::new(
                    [
                        GOAL_BUFFER - PADDLE_WIDTH,
                        (PONG_HEIGHT - PADDLE_HEIGHT) / 2.0,
                    ],
                    PADDLE_HEIGHT,
                    PADDLE_WIDTH,
                ),
                color: GAME_OBJECT_COLOR,
            },
            PaddleType::Right => Paddle {
                motion: MotionPhysics::new(
                    [
                        PONG_WIDTH - GOAL_BUFFER,
                        (PONG_HEIGHT - PADDLE_HEIGHT) / 2.0,
                    ],
                    PADDLE_HEIGHT,
                    PADDLE_WIDTH,
                ),
                color: GAME_OBJECT_COLOR,
            },
        }
    }
}

impl Ball {
    pub fn new() -> Ball {
        let mut ball = Ball {
            motion: MotionPhysics::new([PONG_WIDTH / 2.0, PONG_HEIGHT / 2.0], BALL_SIZE, BALL_SIZE),
            color: GAME_OBJECT_COLOR,
        };
        let range: f64 = thread_rng().gen_range(-BALL_VELOCITY..BALL_VELOCITY);
        ball.motion
            .set_velocity([(7.0 - range.powi(2)).sqrt(), range]);
        ball
    }
}

impl GameObject for Paddle {
    fn get_size(&self) -> [f64; 4] {
        self.motion.get_motion_object().get_size()
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }

    fn update(&mut self) {
        self.motion.update_with_bounds(PONG_HEIGHT, PONG_WIDTH);
    }
}

impl GameObject for Ball {
    fn get_size(&self) -> [f64; 4] {
        self.motion.get_motion_object().get_size()
    }

    fn get_color(&self) -> [f32; 4] {
        self.color
    }

    fn update(&mut self) {
        if let Some(wall) = self.motion.update_with_bounds(PONG_HEIGHT, PONG_WIDTH) {
            match wall {
                kinematics::CollisionWall::Vertical => {
                    self.motion.set_velocity([
                        self.motion.get_velocity()[0],
                        -1.0 * self.motion.get_velocity()[1],
                    ]);
                }
                kinematics::CollisionWall::Horizontal => {
                    self.motion.set_velocity([
                        -1.0 * self.motion.get_velocity()[0],
                        self.motion.get_velocity()[1],
                    ]);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {}
