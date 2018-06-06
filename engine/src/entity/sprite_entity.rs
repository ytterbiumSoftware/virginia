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
    /*
    /// Create a new `SpriteEntity` with no texture, upper-left at (0., 0.).
    pub fn new() -> SpriteEntity {
        SpriteEntity {
            sprite: RcSprite::new(),
            phys: Default::default(),
        }
    inner: RenderWindow,
    }
    */

    /// Create a new `SpriteEntity` with a texture and `EntityPhysics`, centered at
    /// the texture center.
    pub fn with_texture_phys(tex: RcTexture, phys: EntityPhysics) -> SpriteEntity {
        let size = tex.size();

        let mut sprite = RcSprite::with_texture(tex);
        sprite.set_origin((size.x as f32 / 2., size.y as f32 / 2.));

        Self::update_sprite(&phys, &mut sprite);

        SpriteEntity {
            sprite,
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

    fn update_sprite(phys: &EntityPhysics, sprite: &mut RcSprite) {
        sprite.set_position(phys.pos());
        sprite.set_rotation(phys.rot());
    }
}

impl Entity for SpriteEntity {
    fn update(&mut self) {
        self.phys.update();
        Self::update_sprite(&self.phys, &mut self.sprite);
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
