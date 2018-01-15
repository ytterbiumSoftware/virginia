//! Managing and drawing the background.

use sfml::graphics::{Drawable, RenderStates, RenderTarget, Sprite, TextureRef};

pub struct Background<'s> {
    sprite: Sprite<'s>,
}

impl<'s> Background<'s> {
    pub fn new(texture: &'s TextureRef) -> Background {
        Background {
            sprite: Sprite::with_texture(texture),
        }
    }
}

impl<'s> Drawable for Background<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture> (
            &'a self,
            target: &mut RenderTarget,
            states: RenderStates<'texture, 'shader, 'shader_texture>) {
        target.draw_sprite(&self.sprite, states)
    }
}
