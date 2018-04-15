//! Managing and defining generic game objects.

pub use self::sprite_entity::SpriteEntity;

pub mod sprite_entity;

pub const TICKS_SEC: u32 = 45;

use sfml::system::Vector2f;

/// The transformation on a entity, as well as it's velocity.
#[derive(Debug)]
pub struct EntityPhysics {
    /// Current position.
    pos: Vector2f,

    // Momentum - velocity * mass.
    momentum: Vector2f,

    /// Velocity - change in `pos` per unit time. Must be kept consistent with momentum.
    vel: Vector2f,

    // Force - the force experienced by the object this frame.
    force: Vector2f,

    // Mass - the mass of the object.
    mass: f32,
}

impl EntityPhysics {
    /// Create a new entity physics component that has all values except for mass zeroed.
    pub fn new(mass: f32) -> EntityPhysics {
        EntityPhysics {
            pos: Vector2f::new(0., 0.),
            momentum: Vector2f::new(0., 0.),
            vel: Vector2f::new(0., 0.),
            force: Vector2f::new(0., 0.),
            mass,
        }
    }

    /// Simulate one frame. This will set ``self.force`` to ``(0., 0.)``.
    pub fn update(&mut self) {
        self.momentum += self.force;

        // p = mv
        // vm = p
        // v = p / m
        self.vel = self.momentum / self.mass;

        self.pos += self.vel;

        // Reset the force for next frame.
        self.force = Vector2f::new(0., 0.);
    }

    /// Apply a force to the object at the object's center.
    /// This will cause linear motion, and will not induce torque or rotation.
    pub fn apply_force<T: Into<Vector2f>>(&mut self, f: T) {
        self.force += f.into();
    }

    /// Current position.
    pub fn pos(&self) -> Vector2f {
        self.pos
    }

    /// Velocity - change in `pos` per unit time.
    pub fn vel(&self) -> Vector2f {
        self.vel
    }
}

/*
impl Default for EntityPhysics {
    fn default() -> EntityPhysics {
        EntityPhysics {
            pos: Vector2f::new(0., 0.),
            vel: Vector2f::new(0., 0.),
            force: Vector2f::new(0., 0.),
            mass: 1.,
        }
    }
}
*/

/// Functionality of an entity.
pub trait Entity {
    /// Perform per-frame logic.
    fn update(&mut self);

    /// Access the physics component.
    fn phys(&self) -> &EntityPhysics;

    /// Mutably access the physics component.
    fn phys_mut(&mut self) -> &mut EntityPhysics;
}
