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

    // // Velocity - change in pos per unit time. Must be kept consistent with momentum.
    // vel: Vector2f,

    // Force - the force experienced by the object this frame.
    force: Vector2f,

    // Linear Damping - the fraction of velocity lost per unit time.
    linear_damping: f32,

    // Mass - the mass of the object.
    mass: f32,

    /// Current rotation.
    rot: f32,

    // Angular momentum.
    angular_momentum: f32,

    // Torque - analog of force for rotation.
    torque: f32,

    // Angular Damping - the fraction of rotational speed lost per unit time.
    angular_damping: f32,

    // Rotational inertia.
    rotational_inertia: f32,
}

impl EntityPhysics {
    /// Create a new entity physics component that has all values except mass
    /// and rotational inertia zeroed.
    pub fn new(mass: f32, rotational_inertia: f32) -> EntityPhysics {
        Self::with_damping(mass, rotational_inertia, 0., 0.)
    }

    /// Create a new entity physics component using mass, rotational inertia,
    /// linear damping, and angular damping. Other properties will be zeroed.
    pub fn with_damping(mass: f32, rotational_inertia: f32,
                        linear_damping: f32, angular_damping: f32) -> EntityPhysics {
        Self::with_damping_pos(mass, rotational_inertia, linear_damping, angular_damping,
                               Vector2f::new(0., 0.))
    }

    /// Create a new entity physics component at a given position, mass, and rotational_inertia
    /// using linear damping and angular damping. Other properties will be set to zero.
    pub fn with_damping_pos<T: Into<Vector2f>>(mass: f32, rotational_inertia: f32,
                                               linear_damping: f32, angular_damping: f32, pos: T)
                                               -> EntityPhysics {
        EntityPhysics {
            pos: pos.into(),
            momentum: Vector2f::new(0., 0.),
            force: Vector2f::new(0., 0.),
            linear_damping,
            mass,
            rot: 0.,
            angular_momentum: 0.,
            torque: 0.,
            rotational_inertia,
            angular_damping,
        }
    }

    /// Simulate one frame. This will set ``self.force`` to ``(0., 0.)``.
    pub fn update(&mut self) {
        //
        // Linear
        //
        self.momentum += self.force;
        self.momentum *= 1.0 - self.linear_damping;

        // p = mv
        // vm = p
        // v = p / m
        let vel = self.momentum / self.mass;

        self.pos += vel;

        // Reset the force for the next frame.
        self.force = Vector2f::new(0., 0.);

        //
        // Angular
        //
        self.angular_momentum += self.torque;
        self.angular_momentum *= 1.0 - self.angular_damping;

        // L = Iω
        // ωI = L
        // ω = L / I
        let ang_vel = self.angular_momentum / self.rotational_inertia;

        self.rot += ang_vel;
        self.clamp_rot();

        // Reset the torque for the next frame.
        self.torque = 0.;
    }

    /// Apply a force to the object at the object's center.
    /// This will cause linear motion, and will not induce torque or rotation.
    pub fn apply_force<T: Into<Vector2f>>(&mut self, f: T) {
        self.force += f.into();
    }

    /// Apply a force at a point in world coords.
    /// This will induce linear motion **and** rotational motion.
    pub fn apply_force_at<T1, T2>(&mut self, force: T1, point: T2)
        where T1: Into<Vector2f>,
              T2: Into<Vector2f>
    {
        // This is just the force and point as Vector2f's. For convenience tuples
        // can be passed to this method and they will be converted to a Vector2f.
        let force = force.into();
        let point = point.into();
        let center = self.pos;

        self.apply_force(force);

        let dist = point - center;
        self.apply_torque(-((force.x * dist.y) - (force.y * dist.x)));
    }

    /// Apply a torque.
    /// This will induce only rotational motion.
    pub fn apply_torque(&mut self, t: f32) {
        self.torque += t;
    }

    /// Set the position of the object directly.
    pub fn set_position<T: Into<Vector2f>>(&mut self, pos: T) {
        self.pos = pos.into();
    }

    /// Set the rotation of the object directly.
    pub fn set_rotation(&mut self, rot: f32) {
        self.rot = rot;
        self.clamp_rot();
    }

    /// Set linear damping - the fraction of velocity lost per unit time.
    pub fn set_linear_damping(&mut self, linear_damping: f32) {
        self.linear_damping = linear_damping;
    }

    /// Set angular damping - the fraction of rotational speed lost per unit time.
    pub fn set_angular_damping(&mut self, angular_damping: f32) {
        self.angular_damping = angular_damping;
    }

    /// Convenience method to set the damping values for both linear and rotational motion.
    pub fn set_damping(&mut self, linear_damping: f32, angular_damping: f32) {
        self.set_linear_damping(linear_damping);
        self.set_angular_damping(angular_damping);
    }

    /// Current position.
    pub fn pos(&self) -> Vector2f {
        self.pos
    }

    /// Current rotation.
    pub fn rot(&self) -> f32 {
        self.rot
    }

    /// Return linear damping.
    pub fn linear_damping(&self) -> f32 {
        self.linear_damping
    }

    /// Return angular damping.
    pub fn angular_damping(&self) -> f32 {
        self.angular_damping
    }

    /*
    /// Velocity - change in `pos` per unit time.
    pub fn vel(&self) -> Vector2f {
        self.vel
    }
    */

    fn clamp_rot(&mut self) {
        if self.rot > 360. {
            self.rot -= 360.;
        } else if self.rot < -360. {
            self.rot += 360.;
        }
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
