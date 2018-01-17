//! Managing and drawing the background.

use sfml::graphics::{Drawable, RenderStates, RenderTarget};
use refcounted::{RcSprite, RcTexture};

pub struct Background {
    sprite: RcSprite,
}

impl Background {
    pub fn new(texture: RcTexture) -> Background {
        Background {
            sprite: RcSprite::with_texture(texture),
        }
    }
}

impl Drawable for Background {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture> (
            &'a self,
            target: &mut RenderTarget,
            states: RenderStates<'texture, 'shader, 'shader_texture>) {
        unsafe {
            target.draw_sprite(self.sprite.inner(), states);
        }
    }
}
