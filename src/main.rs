extern crate sfml;
extern crate engine;

use std::time::Instant;
use sfml::graphics::{BlendMode, Color, RenderStates, RenderTarget};
use sfml::graphics::blend_mode::Equation;
use sfml::window::Event;
use engine::background::{BackdropKind, BackgroundBuilder};
use engine::entity::{TICKS_SEC, Entity, EntityPhysics, SpriteEntity};
use engine::input::Inputs;
//use engine::refcounted::RcSprite;
use engine::resources::{ResourceId, Resources, TexOptions};
//use engine::starfield;
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

    let mut s_entity = SpriteEntity::with_texture_phys(
        res.textures().get(TextureId::Spaceship0).unwrap(),
        EntityPhysics::new(1., 1.));

    //let original_view = win.view().to_owned();

    let mut last_tick: u64 = 0;
    let begin = Instant::now();
    'game: loop {
        let since = Instant::now().duration_since(begin);
        let final_tick_frame = (since.as_secs() as f64 +
                                since.subsec_nanos() as f64 * 1e-9 * TICKS_SEC as f64) as u64;
        for _ in last_tick..final_tick_frame {
            //println!("{} {}", last_tick, i);
            s_entity.update();
        }
        last_tick = final_tick_frame;

        bg.scroll(win.view());

        while let Some(ev) = win.poll_event() {
            match ev {
                Event::Closed => break 'game,
                _ => {},
            }
        }

        let keys = Inputs::current(&win).keys;

        if keys.right {
            s_entity.phys_mut().apply_force((0.05, 0.));
        }

        if keys.left {
            s_entity.phys_mut().apply_force((-0.05, 0.));
        }

        if keys.up {
            s_entity.phys_mut().apply_force((0., -0.05));
        }

        if keys.down {
            s_entity.phys_mut().apply_force((0., 0.05));
        }

        if keys.a {
            s_entity.phys_mut().apply_torque(-0.01);
        }

        if keys.d {
            s_entity.phys_mut().apply_torque(0.01);
        }

        win.clear(&Color::BLACK);
        win.reset_view();
        win.draw_with_renderstates(&bg, RenderStates {
            blend_mode: BlendMode {
                alpha_equation: Equation::ReverseSubtract,
                ..Default::default()
            },
            ..Default::default()
        });
        win.center_view_on(&s_entity);
        //win.draw(&tester);
        win.draw(&s_entity);
        win.display();
    }
}
