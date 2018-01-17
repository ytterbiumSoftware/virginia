extern crate sfml;
extern crate engine;

use std::rc::Rc;
use sfml::graphics::{Color, RenderTarget, Texture};
use sfml::window::{Event, Key};
use engine::background::Background;
//use engine::refcounted::RcSprite;
use engine::resources::{ResourceId, Resources};
use engine::window::GameWindow;

#[derive(Clone, Copy)]
enum TextureId {
    SpaceLayer0,
}

impl ResourceId for TextureId {
    fn resource_id(&self) -> usize {
        *self as usize
    }
}

fn main() {
    let mut win = GameWindow::new((800, 600), "window");

    //let tex = Rc::new(Texture::from_file("media/tex.png").unwrap());

    let mut res = Resources::new();
    res.textures_mut()
       .add(TextureId::SpaceLayer0,
            Rc::new(Texture::from_file("media/SpaceLayer0.png").unwrap()));

    let bg = Background::new(res.textures().get(TextureId::SpaceLayer0).unwrap());

    //let mut tester = RcSprite::with_texture(tex.clone());
    //let mut tester = RcSprite::new();
    //tester.set_texture(tex.clone(), true);
    //tester.set_scale((10., 10.));

    'game: loop {
        win.clear(&Color::WHITE);
        win.draw(&bg);
        //win.draw(&tester);
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
