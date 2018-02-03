extern crate sfml;
extern crate engine;

use std::time::Instant;
use sfml::graphics::{BlendMode, Color, RenderStates, RenderTarget};
use sfml::graphics::blend_mode::Equation;
use sfml::window::{Event, Key};
use engine::background::{BackdropKind, BackgroundBuilder};
use engine::entity::{Entity, EntityPhysics, SpriteEntity};
//use engine::refcounted::RcSprite;
use engine::resources::{ResourceId, Resources, TexOptions};
use engine::starfield;
use engine::window::GameWindow;

const SIZE: (u32, u32) = (800, 600);
const BG_ALPHA: u8 = 128;

#[derive(Clone, Copy)]
enum TextureId {
    Layer0,
    Layer1,
    Layer2,
    Spaceship0,
}

impl ResourceId for TextureId {
    fn resource_id(&self) -> usize {
        *self as usize
    }
}

fn main() {
    let mut win = GameWindow::new(SIZE, "window");

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
    res.load_tex(TextureId::Spaceship0, "media/Spaceship0.png", &Default::default());

    //let bg = Background::new(res.textures().get(TextureId::SpaceLayer0).unwrap());

    //let mut tester = RcSprite::with_texture(tex.clone());
    //let mut tester = RcSprite::new();
    //tester.set_texture(tex.clone(), true);
    //tester.set_scale((10., 10.));

    //let star = starfield::gen_stars_gas_rctex(SIZE);

    let bd_kind = BackdropKind::LinearGradient(Color::rgb(4, 6, 42), Color::rgb(51, 14, 35));
    let mut bg = BackgroundBuilder::new(win.view(), bd_kind)
                                    //.add(star, 0., 255)
                                    .add(res.textures().get(TextureId::Layer0).unwrap(), 0.125, BG_ALPHA)
                                    .add(res.textures().get(TextureId::Layer1).unwrap(), 0.25, BG_ALPHA)
                                    .add(res.textures().get(TextureId::Layer2).unwrap(), 1., BG_ALPHA)
                                    .build();

    let mut s_entity = SpriteEntity::with_texture_phys(res.textures().get(TextureId::Spaceship0).unwrap(),
                                                       EntityPhysics {
                                                           pos: (400., 350.).into(),
                                                           vel: (5., 5.).into(),
                                                           acc: (0.5, 0.5).into(),
                                                           ..Default::default()
                                                       });

    let mut delta = 0.;
    'game: loop {
        let begin = Instant::now();

        win.clear(&Color::BLACK);
        win.draw_with_renderstates(&bg, RenderStates {
            blend_mode: BlendMode {
                alpha_equation: Equation::ReverseSubtract,
                ..Default::default()
            },
            ..Default::default()
        });
        //win.draw(&tester);
        win.draw(&s_entity);
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

        s_entity.update(delta);

        let delta_dur = Instant::now() - begin;
            delta = delta_dur.as_secs() as f32
                    + delta_dur.subsec_nanos() as f32 * 1e-9;
    }
}
