//! System windowing.

use std::ops::{Deref, DerefMut};
use sfml::graphics::{RenderTarget, RenderWindow, View};
use sfml::window::VideoMode;
use entity::Entity;

const BPP: u32 = 32;

/// Game window, wrapper over `SFML`.
pub struct GameWindow {
    inner: RenderWindow,
    original_view: View,
}

impl GameWindow {
    /// Create new window, and show it immediately.
    pub fn new(size: (u32, u32), name: &str) -> GameWindow {
        let mut inner = RenderWindow::new(VideoMode::new(size.0, size.1, BPP),
                                          name, Default::default(), &Default::default());

        inner.set_vertical_sync_enabled(true);

        let original_view = inner.default_view().to_owned();

        GameWindow {
            inner,
            original_view,
        }
    }

    /// Center the window's view on a particular ``Entity``
    /// Note that this method will center the entity in relation
    /// to the position determined by the physics component.
    /// This position is not necessarilly the entity's center.
    pub fn center_view_on<E: Entity>(&mut self, entity: &E) {
        let mut view = self.inner.view().to_owned();

        let phys = entity.phys();
        view.set_center((phys.pos().x, phys.pos().y));

        self.inner.set_view(&view);
    }

    /// Reset the window's view to the view provided by SFML's
    /// ``default_view()``.
    pub fn reset_view(&mut self) {
        self.inner.set_view(&self.original_view);
    }
}

impl Deref for GameWindow {
    type Target = RenderWindow;

    fn deref(&self) -> &RenderWindow {
        &self.inner
    }
}

impl DerefMut for GameWindow {
    fn deref_mut(&mut self) -> &mut RenderWindow {
        &mut self.inner
    }
}
