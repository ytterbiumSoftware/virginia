//! An entity that uses `RcSprite`.

use sfml::graphics::{Drawable, RenderStates, RenderTarget, Transformable};
use super::{Entity, EntityPhysics};
use refcounted::{RcSprite, RcTexture};

/// A visable entity that is composed of an `RcSprite`
/// and an `EntityTransform`.

pub struct SpriteEntity {
    sprite: RcSprite,
    phys: EntityPhysics,
}

impl SpriteEntity {
    /// Create a new `SpriteEntity` with no texture, upper-left at (0., 0.).
    pub fn new() -> SpriteEntity {
        SpriteEntity {
            sprite: RcSprite::new(),
            phys: Default::default(),
        }
    }

    /// Create a new `SpriteEntity` with a texture, upper-left at (0., 0.).
    pub fn with_texture(tex: RcTexture) -> SpriteEntity {
        SpriteEntity {
            sprite: RcSprite::with_texture(tex),
            phys: Default::default(),
        }
    }

    /// Create a new `SpriteEntity` with a texture and `EntityPhysics`.
    pub fn with_texture_phys(tex: RcTexture, phys: EntityPhysics) -> SpriteEntity {
        SpriteEntity {
            sprite: RcSprite::with_texture(tex),
            phys,
        }
    }

    /// Access the inner `RcSprite`.
    pub fn rc_sprite(&self) -> &RcSprite {
        &self.sprite
    }

    /// Mutably access the inner `RcSprite`.
    pub fn rc_sprite_mut(&mut self) -> &mut RcSprite {
        &mut self.sprite
    }
}

impl Entity for SpriteEntity {
    fn update(&mut self) {
        self.phys.update();
        self.sprite.set_position(self.phys.pos);
        self.sprite.set_rotation(self.phys.rot);
    }

    fn phys(&self) -> &EntityPhysics {
        &self.phys
    }

    fn phys_mut(&mut self) -> &mut EntityPhysics {
        &mut self.phys
    }
}

impl Drawable for SpriteEntity {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture> (
            &'a self,
            target: &mut RenderTarget,
            states: RenderStates<'texture, 'shader, 'shader_texture>) {
        self.sprite.draw(target, states);
    }
}
