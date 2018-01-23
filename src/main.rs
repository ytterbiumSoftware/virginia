extern crate sfml;
extern crate engine;

use sfml::graphics::{BlendMode, Color, RenderStates, RenderTarget};
use sfml::graphics::blend_mode::Equation;
use sfml::window::{Event, Key};
use engine::background::{BackdropKind, BackgroundBuilder};
//use engine::refcounted::RcSprite;
use engine::resources::{ResourceId, Resources, TexOptions};
use engine::window::GameWindow;

#[derive(Clone, Copy)]
enum TextureId {
    Layer0,
    Layer1,
    Layer2,
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

    res.load_tex(TextureId::Layer0, "media/CloudLayer0.png", TexOptions::build().repeated().smooth());
    res.load_tex(TextureId::Layer1, "media/CloudLayer1.png", TexOptions::build().repeated().smooth());
    res.load_tex(TextureId::Layer2, "media/CloudLayer2.png", TexOptions::build().repeated().smooth());

    //let bg = Background::new(res.textures().get(TextureId::SpaceLayer0).unwrap());

    //let mut tester = RcSprite::with_texture(tex.clone());
    //let mut tester = RcSprite::new();
    //tester.set_texture(tex.clone(), true);
    //tester.set_scale((10., 10.));

    let bd_kind = BackdropKind::LinearGradient(Color::rgb(4, 6, 42), Color::rgb(51, 14, 35));
    let mut bg = BackgroundBuilder::new(win.view(), bd_kind)
                                    .add(res.textures().get(TextureId::Layer0).unwrap(), 0.125)
                                    .add(res.textures().get(TextureId::Layer1).unwrap(), 0.25)
                                    .add(res.textures().get(TextureId::Layer2).unwrap(), 1.)
                                    .build();

    'game: loop {
        win.clear(&Color::BLACK);
        win.draw_with_renderstates(&bg, RenderStates {
            blend_mode: BlendMode {
                alpha_equation: Equation::ReverseSubtract,
                ..Default::default()
            },
            ..Default::default()
        });
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
