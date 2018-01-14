extern crate sfml;
extern crate engine;

use sfml::graphics::{Color, RenderTarget};
use sfml::window::{Event, Key};
use engine::window::GameWindow;

fn main() {
    let mut win = GameWindow::new((800, 600), "window");

    'game: loop {
        win.clear(&Color::WHITE);
        win.display();

        while let Some(ev) = win.poll_event() {
            match ev {
                Event::KeyPressed { code: Key::Escape, .. } => break 'game,
                Event::Closed => break 'game,
                _ => {},
            }
        }
    }
}
