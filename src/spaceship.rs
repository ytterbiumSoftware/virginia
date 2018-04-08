use engine::entity::SpriteEntity;
use engine::resources::Resources;
use super::TextureId;

pub struct Spaceship {
    entity: SpriteEntity,
}

impl Spaceship {
    pub fn new(res: &Resources) -> Spaceship {
        Spaceship {
            entity: SpriteEntity::with_texture(res.textures().get(TextureId::Spaceship0).unwrap()),
        }
    }
}
