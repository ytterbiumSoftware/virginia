//! Managing and defining generic game objects.

pub use self::sprite_entity::SpriteEntity;

pub mod sprite_entity;

use sfml::system::Vector2f;

/// The transformation on a entity, as well as it's velocity.
#[derive(Debug)]
pub struct EntityPhysics {
    /// Current position.
    pub pos: Vector2f,

    /// Velocity - change in `pos` per unit time.
    pub vel: Vector2f,

    /// Acceleration - change in change per unit time.
    pub acc: Vector2f,

    /// Current rotation **in degrees**.
    pub rot: f32,

    /// Rotational velocity - change in `rot` per unit time **in degrees**.
    pub rot_vel: f32,
}

impl EntityPhysics {
    /// Simulate one frame.
    pub fn update(&mut self, delta: f32) {
        self.vel += self.acc * delta;

        self.pos += self.vel * delta;

        self.rot += self.rot_vel * delta;
    }
}

impl Default for EntityPhysics {
    fn default() -> EntityPhysics {
        EntityPhysics {
            pos: Vector2f::new(0., 0.),
            vel: Vector2f::new(0., 0.),
            acc: Vector2f::new(0., 0.),
            rot: 0.,
            rot_vel: 0.,
        }
    }
}

/// Functionality of an entity.
pub trait Entity {
    /// Perform per-frame logic.
    /// # Arguments
    /// * delta: The amount of time since the last frame in seconds.
    fn update(&mut self, delta: f32);
}
