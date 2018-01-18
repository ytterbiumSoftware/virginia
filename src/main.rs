extern crate sfml;
extern crate engine;

use sfml::graphics::{Color, RenderTarget};
use sfml::window::{Event, Key};
use engine::background::BackgroundBuilder;
//use engine::refcounted::RcSprite;
use engine::resources::{ResourceId, Resources, TexOptions};
use engine::window::GameWindow;

#[derive(Clone, Copy)]
enum TextureId {
    Layer0,
    Layer1,
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

    //let mut tex = Texture::from_file("media/testing_new/Layer0.png").unwrap();
    //tex.set_repeated(true);
    //assert!(!res.textures_mut().add(TextureId::Layer0, Rc::new(tex)));

    //let mut tex = Texture::from_file("media/testing_new/Layer1.png").unwrap();
    //tex.set_repeated(true);
    //assert!(!res.textures_mut().add(TextureId::Layer1, Rc::new(tex)));

    res.load_tex(TextureId::Layer0, "media/testing_new/Layer0.png", TexOptions::build().repeated());
    res.load_tex(TextureId::Layer1, "media/testing_new/Layer1.png", TexOptions::build().repeated());

    //let bg = Background::new(res.textures().get(TextureId::SpaceLayer0).unwrap());

    //let mut tester = RcSprite::with_texture(tex.clone());
    //let mut tester = RcSprite::new();
    //tester.set_texture(tex.clone(), true);
    //tester.set_scale((10., 10.));

    let mut bg = BackgroundBuilder::new()
                                    .add(res.textures().get(TextureId::Layer0).unwrap(), 0.5)
                                    .add(res.textures().get(TextureId::Layer1).unwrap(), 1.)
                                    .build();

    'game: loop {
        win.clear(&Color::WHITE);
        win.draw(&bg);
        //win.draw(&tester);
        win.display();

        while let Some(ev) = win.poll_event() {
            match ev {
                Event::KeyPressed { code: Key::Escape, .. } => break 'game,
                Event::KeyPressed { code, .. } => match code {
                    Key::Up => bg.scroll((0., -5.)),
                    Key::Down => bg.scroll((0., 5.)),
                    Key::Left => bg.scroll((-5., 0.)),
                    Key::Right => bg.scroll((5., 0.)),
                    _ => {},
                },
                Event::Closed => break 'game,
                _ => {},
            }
        }
    }
}
