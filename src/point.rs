use std::fmt;

pub use crate::vec2::Vec2;

pub trait PhTickable {
    fn tick(&mut self, _dt: f32) {
        println!("tick not impld");
    }
}

pub trait PhImpulsable {
    fn apply_impulse(&mut self, dp: Vec2<f32>);
}

pub trait PhGravityAffected: PhImpulsable {
    fn gravitize(&mut self, dt: f32, g: f32) {
        self.apply_impulse(Vec2::new(0.0, g * dt));
    }
}

pub struct Point {
    pub pos: Vec2<f32>,
    pub vel: Vec2<f32>,
    pub mass: f32,
}

impl Point {
    pub fn new(pos: Vec2<f32>, vel: Vec2<f32>, mass: f32) -> Self {
        Self {pos, vel, mass}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{ pos:{}, vel:{}, mass:{} }}", self.pos, self.vel, self.mass)
    }
}

impl PhImpulsable for Point {
    fn apply_impulse(&mut self, dp: Vec2<f32>) {
        self.vel = self.vel + dp / self.mass;
    }
}

impl PhGravityAffected for Point {}

impl PhTickable for Point {
    fn tick(&mut self, dt: f32) {
        self.gravitize(dt, -9.8 * self.mass);

        self.vel = self.vel * (1.0 - 0.0 * dt);
        self.pos = self.pos + self.vel * dt;
        self.pos.y = match self.pos.y < 0.0 {
            true => 0.0,
            false => self.pos.y,
        }
    }
}
