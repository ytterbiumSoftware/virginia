//! Managing and drawing the background.

use sfml::graphics::{Drawable, IntRect, RenderStates, RenderTarget};
use sfml::system::Vector2f;
use refcounted::{RcSprite, RcTexture};

/// A parallax-scrolling background.
pub struct Background {
    layers: Vec<Layer>,
    pos: Vector2f,
}

impl Background {
    /// Create a new background with no layers.
    /// Will only allocate if layers are added.
    pub fn new() -> Background {
        Self::with_num_layers_hint(0)
    }

    /// Create a new background with no layers.
    /// **Allocates** enough memory for `num_layers_hint`
    /// layers.
    pub fn with_num_layers_hint(num_layers_hint: usize) -> Background {
        Background {
            layers: Vec::with_capacity(num_layers_hint),
            pos: (0., 0.).into(),
        }
    }

    /// Add a new layer on top of the existing ones.
    pub fn add_layer_top(&mut self, texture: RcTexture, scroll_coefficent: f32) {
        self.layers.push(Layer::new(texture, scroll_coefficent));
    }

    /// Scroll the background.
    pub fn scroll<V: Into<Vector2f>>(&mut self, scroll: V) {
        self.pos += scroll.into();

        for i in &mut self.layers {
            let rect = i.sprite.texture_rect();

            i.sprite.set_texture_rect(&IntRect::new((self.pos.x * i.coefficent) as i32,
                                                    (self.pos.y * i.coefficent) as i32,
                                                    rect.width, rect.height));
        }
    }
}

impl Drawable for Background {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture> (
            &'a self,
            target: &mut RenderTarget,
            states: RenderStates<'texture, 'shader, 'shader_texture>) {
        for i in &self.layers {
            let states = RenderStates {
                blend_mode: states.blend_mode,
                transform: states.transform,
                texture: states.texture,
                shader: states.shader,
            };

            target.draw_with_renderstates(&i.sprite, states);
        }
    }
}

///// A builder for `Background`.
pub struct BackgroundBuilder {
    inner: Background,
}

impl BackgroundBuilder {
    /// Create a background builder object.
    pub fn new() -> BackgroundBuilder {
        Self::with_num_layers_hint(0)
    }

    /// Create a background builder.
    /// See `Background::with_num_layers_hint`.
    pub fn with_num_layers_hint(num_layers_hint: usize) -> BackgroundBuilder {
        BackgroundBuilder {
            inner: Background::with_num_layers_hint(num_layers_hint),
        }
    }

    /// Add a layer to the top.
    /// See `Background::add_layer_top`.
    pub fn add(mut self, texture: RcTexture, scroll_coefficent: f32) -> BackgroundBuilder {
        self.inner.add_layer_top(texture, scroll_coefficent);
        self
    }

    /// Finish and return the `Background`.
    pub fn build(self) -> Background {
        self.inner
    }
}

struct Layer {
    sprite: RcSprite,
    coefficent: f32,
}

impl Layer {
    fn new(texture: RcTexture, coefficent: f32) -> Layer {
        Layer {
            sprite: RcSprite::with_texture(texture),
            coefficent,
        }
    }
}
