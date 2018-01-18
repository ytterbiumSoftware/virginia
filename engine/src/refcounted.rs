//! Reference counted wrappers over SFML types
//! that borrow values.
//! Keep in mind that this is **totally a hack**
//! (and it is dependent on a specific SFML version).
//! If you suspect memory errors or UB, look here :/

use std::mem;
use std::rc::Rc;
use csfml_graphics_sys as ffi;
use sfml::system::Vector2f;
use sfml::graphics::{Color, Drawable, FloatRect, IntRect, Sprite, RenderStates, RenderTarget,
                     Texture, Transform, Transformable};

/// A reference counted texture.
pub type RcTexture = Rc<Texture>;

// For our purposes we need access to private fields.
// So we are doing a very unsafe cast, because the sprite
// is really implemented in the C++ code, so the `Sprite` `struct`
// in `sfml` is just an opaque pointer to a C++ class instance.
// Don't try this at home. Yuck!
struct ImposterSprite {
    ptr: *mut ffi::sfSprite,
}

// Same story.
struct ImposterTexture {
    ptr: *const ffi::sfTexture,
}

/// A sprite that is reference counted.
pub struct RcSprite {
    tex: Option<RcTexture>,
    inner: Sprite<'static>,
}

impl RcSprite {
    /// Create a new sprite with no texture.
    pub fn new() -> RcSprite {
        RcSprite {
            tex: None,
            inner: Sprite::new(),
        }
    }

    /// Create a new sprite with a texture.
    pub fn with_texture(tex: RcTexture) -> RcSprite {
        let mut rcsprite = Self::new();
        unsafe { rcsprite.do_set_tex(tex, true); }
        rcsprite
    }

    /// Set the sprite's texture.
    pub fn set_texture(&mut self, tex: RcTexture, reset_rect: bool) {
        unsafe { self.do_set_tex(tex, reset_rect); }
    }

    /// Set color multiplier.
    pub fn set_color(&mut self, color: &Color) {
        self.inner.set_color(color);
    }

    /// Return color multiplier.
    pub fn color(&self) -> Color {
        self.inner.color()
    }

    /// Get the local bounding rectangle of a sprite.
    /// The returned rectangle is in local coordinates,
    /// which means that it ignores the transformations
    /// (translation, rotation, scale, ...) that are
    /// applied to the entity.
    pub fn local_bounds(&self) -> FloatRect {
        self.inner.local_bounds()
    }

    /// Get the global bounding rectangle of a sprite
    /// The returned rectangle is in global coordinates,
    /// which means that it takes in account the transformations
    /// (translation, rotation, scale, ...) that are
    /// applied to the entity.
    pub fn global_bounds(&self) -> FloatRect {
        self.inner.global_bounds()
    }

    /// Get the sub-rectangle of the texture displayed by a sprite.
    pub fn texture_rect(&self) -> IntRect {
        self.inner.texture_rect()
    }

    /// The texture rect is useful when you don't want
    /// to display the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    pub fn set_texture_rect(&mut self, rect: &IntRect) {
        self.inner.set_texture_rect(rect);
    }

    /*
    // Return the base SFML sprite. Misuse could cause safety
    // errors, so use carefully.
    pub(crate) unsafe fn inner(&self) -> &Sprite {
        &self.inner
    }
    */

    // Actually set the texture for the implementation of `self.inner`
    // and assign `Some(tex)` to `self.tex`.
    unsafe fn do_set_tex(&mut self, tex: RcTexture, reset_rect: bool) {
        let hack_sprite: &mut ImposterSprite = mem::transmute(&mut self.inner);
        let hack_tex: &ImposterTexture = mem::transmute(&*tex);
        ffi::sfSprite_setTexture(hack_sprite.ptr, hack_tex.ptr, reset_rect as _);
        self.tex = Some(tex);
    }
}

impl<'s> Drawable for RcSprite {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        self.inner.draw(target, states);
    }
}

impl<'s> Transformable for RcSprite {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        self.inner.set_position(position);
    }

    fn set_rotation(&mut self, angle: f32) {
        self.inner.set_rotation(angle);
    }

    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        self.inner.set_scale(scale);
    }

    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        self.inner.set_origin(origin);
    }

    fn position(&self) -> Vector2f {
        self.inner.position()
    }

    fn rotation(&self) -> f32 {
        self.inner.rotation()
    }

    fn get_scale(&self) -> Vector2f {
        self.inner.get_scale()
    }

    fn origin(&self) -> Vector2f {
        self.inner.origin()
    }

    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        self.inner.move_(offset);
    }

    fn rotate(&mut self, angle: f32) {
        self.inner.rotate(angle);
    }

    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        self.inner.scale(factors);
    }

    fn transform(&self) -> Transform {
        self.inner.transform()
    }

    fn inverse_transform(&self) -> Transform {
        self.inner.inverse_transform()
    }
}
