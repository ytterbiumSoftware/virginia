//! Managing and defining generic game objects.

pub use self::sprite_entity::SpriteEntity;

pub mod sprite_entity;

pub const TICKS_SEC: u32 = 45;

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

    /// Rotational acceleration - chance in change per unit time **in degrees**.
    pub rot_acc: f32,
}

impl EntityPhysics {
    /// Simulate one frame.
    pub fn update(&mut self) {
        self.vel += self.acc;

        self.pos += self.vel;

        self.rot_vel += self.rot_acc;

        self.rot += self.rot_vel;

        if self.rot > 360. {
            self.rot -= 360.;
        }
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
            rot_acc: 0.,
        }
    }
}

/// Functionality of an entity.
pub trait Entity {
    /// Perform per-frame logic.
    fn update(&mut self);

    /// Access the physics component.
    fn phys(&self) -> &EntityPhysics;

    /// Mutably access the physics component.
    fn phys_mut(&mut self) -> &mut EntityPhysics;
}
