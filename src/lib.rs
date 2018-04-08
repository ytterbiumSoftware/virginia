extern crate sfml;
extern crate engine;

pub mod spaceship;

use std::error::Error;
use std::fmt::Display;
use engine::resources::ResourceId;

#[derive(Clone, Copy)]
pub enum TextureId {
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

/*
/// If a resource can't be loaded;
#[derive(Debug)]
pub struct ResError;

impl Display for ResError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for ResError {
    fn description(&self) -> &str {
        "Resource could not be loaded"
    }
}
*/
