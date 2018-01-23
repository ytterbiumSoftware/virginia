//! Managing and drawing the background.

use sfml::graphics::{Color, Drawable, IntRect, PrimitiveType, RenderStates, RenderTarget,
                     Transformable, Vertex, VertexArray, ViewRef};
use sfml::system::Vector2f;
use refcounted::{RcSprite, RcTexture};

/// A parallax-scrolling background.
pub struct Background {
    backdrop: Backdrop,
    layers: Vec<Layer>,
    pos: Vector2f,
}

impl Background {
    /// Create a new background with no layers.
    /// Will only allocate if layers are added.
    pub fn new(view: &ViewRef, backdrop_kind: BackdropKind) -> Background {
        Self::with_num_layers_hint(0, view, backdrop_kind)
    }

    /// Create a new background with no layers.
    /// **Allocates** enough memory for `num_layers_hint`
    /// layers.
    pub fn with_num_layers_hint(num_layers_hint: usize, view: &ViewRef,
                                backdrop_kind: BackdropKind) -> Background {
        Background {
            backdrop: Backdrop::new(backdrop_kind, view),
            layers: Vec::with_capacity(num_layers_hint),
            pos: (0., 0.).into(),
        }
    }

    /// Add a new layer on top of the existing ones.
    pub fn add_layer_top(&mut self, view: &ViewRef, texture: RcTexture, scroll_coefficient: f32) {
        self.layers.push(Layer::new(view, texture, scroll_coefficient));
    }

    /// Scroll the background.
    pub fn scroll<V: Into<Vector2f>>(&mut self, scroll: V) {
        self.pos += scroll.into();

        for i in &mut self.layers {
            let rect = i.sprite.texture_rect();

            i.sprite.set_texture_rect(&IntRect::new((self.pos.x * i.coefficient) as i32,
                                                    (self.pos.y * i.coefficient) as i32,
                                                    rect.width, rect.height));
        }
    }
}

impl Drawable for Background {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture> (
            &'a self,
            target: &mut RenderTarget,
            states: RenderStates<'texture, 'shader, 'shader_texture>) {
        let states2 = RenderStates {
            blend_mode: states.blend_mode,
            transform: states.transform,
            texture: states.texture,
            shader: states.shader,
        };
        target.draw_with_renderstates(&self.backdrop, states2);

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
pub struct BackgroundBuilder<'a> {
    inner: Background,
    view: &'a ViewRef,
}

impl<'a> BackgroundBuilder<'a> {
    /// Create a background builder object.
    pub fn new(view: &'a ViewRef, backdrop_kind: BackdropKind) -> BackgroundBuilder {
        Self::with_num_layers_hint(4, view, backdrop_kind)
    }

    /// Create a background builder.
    /// See `Background::with_num_layers_hint`.
    pub fn with_num_layers_hint(num_layers_hint: usize, view: &'a ViewRef,
                                backdrop_kind: BackdropKind) -> BackgroundBuilder {
        BackgroundBuilder {
            inner: Background::with_num_layers_hint(num_layers_hint, view, backdrop_kind),
            view
        }
    }

    /// Add a layer to the top.
    /// See `Background::add_layer_top`.
    pub fn add(mut self, texture: RcTexture, scroll_coefficient: f32) -> BackgroundBuilder<'a> {
        self.inner.add_layer_top(self.view, texture, scroll_coefficient);
        self
    }

    /// Finish and return the `Background`.
    pub fn build(self) -> Background {
        self.inner
    }
}

/// The possibilities for the farthest back part of a `Background`.
#[derive(Clone, Copy, Debug)]
pub enum BackdropKind {
    /// A solid color.
    Solid(Color),

    // /// A vertical top-to-bottom linear gradient.
    LinearGradient(Color, Color),
}

// Private
// #######

struct Layer {
    sprite: RcSprite,
    coefficient: f32,
}

impl Layer {
    fn new(view: &ViewRef, texture: RcTexture, coefficient: f32) -> Layer {
        let tex_size = texture.size();
        let tex_size = Vector2f::new(tex_size.x as f32, tex_size.y as f32);
        let view_size = view.size();
        let mut sprite = RcSprite::with_texture(texture);

        if view_size.x > tex_size.x || view_size.y > tex_size.y {
            sprite.set_scale((view_size.x / tex_size.x,
                              view_size.y / tex_size.y));
        }

        Layer {
            sprite,
            coefficient,
        }
    }
}

struct Backdrop {
    vao: VertexArray,
}

impl Backdrop {
    fn new(kind: BackdropKind, view: &ViewRef) -> Backdrop {
        use self::BackdropKind::*;

        let size = view.size();
        let mut vao = VertexArray::new(PrimitiveType::Triangles, 3);

        let color = match kind {
            Solid(c0) => {
                (c0, c0)
            },
            LinearGradient(c0, c1) => {
                (c0, c1)
            }
        };

        vao.append(&Vertex::with_pos_color((0.,     size.y), color.1));
        vao.append(&Vertex::with_pos_color((size.x, size.y), color.1));
        vao.append(&Vertex::with_pos_color((size.x, 0.),     color.0));

        vao.append(&Vertex::with_pos_color((0.,     0.),     color.0));
        vao.append(&Vertex::with_pos_color((0.,     size.y), color.1));
        vao.append(&Vertex::with_pos_color((size.x, 0.),     color.0));

        Backdrop {
            vao,
        }
    }
}

impl Drawable for Backdrop {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture> (
            &'a self,
            target: &mut RenderTarget,
            states: RenderStates<'texture, 'shader, 'shader_texture>) {
        target.draw_with_renderstates(&self.vao, states);
    }
}
