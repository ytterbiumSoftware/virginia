//! This module provides support for large space bodies that are renderable
//! and induce a large gravitational force.

use sfml::graphics::{Drawable, RenderStates, RenderTarget};
use sfml::system::Vector2f;
use entity::{Entity, EntityPhysics, SpriteEntity};
use refcounted::RcTexture;
use util::Vector2Ext;

const GRAVITY_CUTOFF: f32 = 100.;
const GRAVITATIONAL_CONSTANT: f32 = 5.0e-3;

/// Drawable collection of planets.
pub struct PlanetManager {
    planets: Vec<Planet>,
}

impl PlanetManager {
    /// Create a ``PlanetManager`` with no planets.
    pub fn new() -> PlanetManager {
        PlanetManager {
            planets: Vec::new(),
        }
    }

    /// Add a planet to the manager, transferring ownership.
    pub fn add_planet(&mut self, planet: Planet) {
        self.planets.push(planet);
    }

    /// Apply gravity of planets to an ``Entity``.
    pub fn affect_entity<E: Entity>(&self, entity: &mut E) {
        let entity_phys = entity.phys_mut();
        let entity_pos = entity_phys.pos();
        let entity_mass = entity_phys.mass();

        for i in &self.planets {
            let difference = i.entity.phys().pos() - entity_pos;
            let difference_len_sq = difference.length_sq();
            if difference_len_sq > GRAVITY_CUTOFF {
                let magnitude_sq_recip = 1. / difference_len_sq;
                if !magnitude_sq_recip.is_infinite() {
                    //println!("{:?}", magnitude_sq_recip);
                    entity_phys.apply_force(difference * i.entity.phys().mass() * entity_mass
                        * magnitude_sq_recip * GRAVITATIONAL_CONSTANT);
                }
            }
        }

        /*
        let entity_phys = entity.phys_mut();
        let entity_pos = entity_phys.pos();
        let entity_mass = entity_phys.mass();

        for i in &self.planets {
            let difference = i.entity.phys().pos() - entity_pos;
            let distance_sq = difference.length_sq();
            let magnitude = (i.entity.phys().mass() * entity_mass) / distance_sq;
            if !magnitude.is_infinite() {
                entity_phys.apply_force(difference.normalize() * magnitude * 1.0e-2);
            }
            //println!("magnitude: {:?}", magnitude);

            //     Gmm
            // F = ---
            //     r^2
            //

            //phys.apply_force();
        }
        */
    }
}

impl Drawable for PlanetManager {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture> (
            &'a self,
            target: &mut RenderTarget,
            states: RenderStates<'texture, 'shader, 'shader_texture>) {
        for i in &self.planets {
            let states = RenderStates {
                blend_mode: states.blend_mode,
                transform: states.transform,
                texture: states.texture,
                shader: states.shader,
            };

            target.draw_with_renderstates(&i.entity, states);
        }
    }
}

/// Represents an individual planet.
pub struct Planet {
    entity: SpriteEntity,
}

impl Planet {
    /// Create a new planet with a texture, mass, and position in world coordinates.
    pub fn new<V: Into<Vector2f>>(texture: RcTexture, mass: f32, pos: V) -> Planet {
        let phys = EntityPhysics::with_damping_pos(mass, 1., 0., 0., pos);

        Planet {
            entity: SpriteEntity::with_texture_phys(texture, phys),
        }
    }
}
