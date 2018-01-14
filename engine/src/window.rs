//! System windowing.

use std::ops::{Deref, DerefMut};
use sfml::graphics::{RenderWindow};
use sfml::window::VideoMode;

const BPP: u32 = 32;

/// Game window, wrapper over `SFML`.
pub struct GameWindow(RenderWindow);

impl GameWindow {
    /// Create new window, and show it immediately.
    pub fn new(size: (u32, u32), name: &str) -> GameWindow {
        let mut g = GameWindow(RenderWindow::new(VideoMode::new(size.0, size.1, BPP),
                   name, Default::default(), &Default::default()));

        g.set_vertical_sync_enabled(true);

        g
    }
}

impl Deref for GameWindow {
    type Target = RenderWindow;

    fn deref(&self) -> &RenderWindow {
        &self.0
    }
}

impl DerefMut for GameWindow {
    fn deref_mut(&mut self) -> &mut RenderWindow {
        &mut self.0
    }
}
