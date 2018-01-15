extern crate sfml;
extern crate engine;

use std::rc::Rc;
use sfml::graphics::{Color, RenderTarget, Texture, Transformable};
use sfml::window::{Event, Key};
use engine::background::Background;
use engine::refcounted::RcSprite;
use engine::window::GameWindow;

fn main() {
    let mut win = GameWindow::new((800, 600), "window");

    let tex = Rc::new(Texture::from_file("media/tex.png").unwrap());

    let bgtex = Texture::from_file("media/SpaceLayer0.png").unwrap();
    let bg = Background::new(&bgtex);

    let mut tester = RcSprite::with_texture(tex.clone());
    tester.set_scale((10., 10.));

    'game: loop {
        win.clear(&Color::WHITE);
        win.draw(&bg);
        win.draw(&tester);
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
