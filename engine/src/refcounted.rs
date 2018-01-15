//! Reference counted wrappers over SFML types
//! that borrow values.
//! Keep in mind that this is **totally a hack**
//! (and it is dependent on a specific SFML version).
//! If you suspect memory errors or UB, look here :/

use std::mem;
use std::rc::Rc;
use csfml_graphics_sys as ffi;
use sfml::system::Vector2f;
use sfml::graphics::{Drawable, Sprite, RenderStates, RenderTarget, Texture, Transform,
                     Transformable};

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
    ptr: *mut ffi::sfTexture,
}

/// A sprite that is reference counted.
pub struct RcSprite {
    _tex: Rc<Texture>,
    inner: Sprite<'static>,
}

impl RcSprite {
    /// Create a new sprite with a texture.
    pub fn with_texture(tex: Rc<Texture>) -> RcSprite {
        let inner = Sprite::new();
        unsafe {
            let hack_sprite: &ImposterSprite = mem::transmute(&inner);
            let hack_tex: &ImposterTexture = mem::transmute(&*tex);
            ffi::sfSprite_setTexture(hack_sprite.ptr, hack_tex.ptr, true as _);
        }

        RcSprite {
            _tex: tex,
            inner,
        }
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
