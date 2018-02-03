//! Randomly generated star/space backgrounds.

use std::rc::Rc;
use std::time::{UNIX_EPOCH, SystemTime};
use noise::{NoiseModule, Perlin, Seedable};
use sfml::graphics::{Color, Image, Texture};

// In case `SystemTime` reports `Err`, we have a seed.
const BACKUP_SEED: usize = 547879234768908253;

/* // How much to scale down the precison of noise from the supplied size.
const SCALE_DOWN: u32 = 10; */

// The threshold above which a gas pixel is placed.
const GAS_THRESHOLD: f32 = 0.35;

// The threshold above which a star pixel is placed.
const STAR_THRESHOLD: f32 = 0.42;

// The density of the stars.
const STAR_DENSITY: f32 = 20.;

// The multiplier on the noise input.
const MULTI: f32 = 0.002;

// Color of gas clouds.
const GAS_COLOR: Color = Color { r: 255, g: 255, b: 255, a: 2 };

// How much the noise influences the color.
const COLOR_INFLUENCE: f32 = 3.;

// Lower half range of the perlin function.
const RANGE_OFFSET: f32 = 0.7072;

// Color of stars at their edges.
const STAR_ALPHA_NEAR: u8 = 220;
const STAR_ALPHA_FAR: u8 = 50;

/// Generate stars and gas on a transparent background.
/// # Panics
/// Panics if the texture could not be created.
pub fn gen_stars_gas(size: (u32, u32)) -> Image {
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as usize,
        Err(_) => BACKUP_SEED,
    };

    let perlin = Perlin::new().set_seed(seed);

    let mut img = Image::new(size.0, size.1);

    for x in 0..size.0 {
        for y in 0..size.1 {
            let (fx, fy) = (x as f32, y as f32);
            let (fxm, fym) = (fx * MULTI, fy * MULTI);

            let value = perlin.get([fxm, fym]);
            if value > GAS_THRESHOLD {
                let mut color = GAS_COLOR;
                color.a += ((value + RANGE_OFFSET) * COLOR_INFLUENCE) as u8;

                img.set_pixel(x, y, &color);

                let star_factor = perlin.get([fxm * 10., fym * 10.]);
                //let (nx, ny) = (x as i32, y as i32);
                if x >= 2 && y >= 2 &&
                        star_factor > STAR_THRESHOLD &&
                        (1. / fx.cos() * 26.).round() % STAR_DENSITY == 0.
                        && fy % STAR_DENSITY == 0. {
                    for dx in 0..3 {
                        for dy in 0..3 {
                            let color = if dx == 1 && dy == 1 {
                                Color::rgba(255, 255, 255, STAR_ALPHA_NEAR)
                            } else {
                                Color::rgba(255, 255, 255, STAR_ALPHA_FAR)
                            };

                            img.set_pixel(x - dx, y - dy, &color);
                        }
                    }
                }
            } else {
                img.set_pixel(x, y, &Color::TRANSPARENT);
            }

            /*
            let (nx, ny) = (x as i32, y as i32);
            if x > 2 && y > 2 &&
                    (nx + ny) % STAR_DENSITY == 0 && (nx - ny) % STAR_DENSITY == 0 &&
                    value > STAR_THRESHOLD {
                for dx in -2..1 {
                    for dy in -2..1 {
                        let color = if dx == -1 && dy == -1 {
                            Color::WHITE
                        } else if dx + dy == 0 || dx + dy == -4 {
                            Color::rgba(255, 255, 255, STAR_ALPHA_NEAR)
                        } else {
                            if value > STAR_THRESHOLD + STAR_THRESHOLD / 2. {
                                Color::rgba(255, 255, 255, STAR_ALPHA_FAR)
                            } else {
                                Color::TRANSPARENT
                            }
                        };

                        img.set_pixel((nx + dx) as u32, (ny + dy) as u32, &color);
                    }
                }
            }
            */
        }
    }

    img

    /*
    let mut img = Image::new(size.0, size.1);
    for x in 0..size.0 {
        for y in 0..size.1 {
            img.set_pixel(x, y, &GAS_COLOR);
        }
    }

    let iters = (size.0 / SCALE_DOWN, size.1 / SCALE_DOWN);

    for x in 0..iters.0 {
        for y in 0..iters.1 {
            let (x, y) = (x * SCALE_DOWN, y * SCALE_DOWN);
            let (fx, fy) = (x as f32 * MULTI, y as f32 * MULTI);

            if x >= 5 && y >= 5 && x + 5 < size.0 && y + 5 < size.1 && perlin.get([fx, fy]) > THRESHOLD {
                for dx in -5..6 {
                    for dy in -5..6 {
                        img.set_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32, &Color::TRANSPARENT);
                    }
                }
            }
        }
    }

    img
    */
}

/// Convenience function that produces a `Texture`
/// from `gen_stars_gas`.
#[inline]
pub fn gen_stars_gas_tex(size: (u32, u32)) -> Texture {
    Texture::from_image(&gen_stars_gas(size)).unwrap()
}

/// Convenience function that produces a `Rc<Texture>`
/// (`RcTexture`) from `gen_stars_gas_tex`.
#[inline]
pub fn gen_stars_gas_rctex(size: (u32, u32)) -> Rc<Texture> {
    Rc::new(gen_stars_gas_tex(size))
}
