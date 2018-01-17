//! Managing textures, etc.

use std::collections::HashMap;
use refcounted::RcTexture;

/// The top-level manager for all media resources.
pub struct Resources {
    textures: TextureManager,
}

impl Resources {
    /// Create a new top-level resource manager.
    pub fn new() -> Resources {
        Resources {
            textures: TextureManager::new(),
        }
    }

    /// Return a reference to the texture manager.
    pub fn textures(&self) -> &TextureManager {
        &self.textures
    }

    /// Returns a mutable reference to the texture manager.
    pub fn textures_mut(&mut self) -> &mut TextureManager {
        &mut self.textures
    }
}

/// Resource manager for textures.
pub type TextureManager = ResourceManager<RcTexture>;

/// A `trait` for any ID that can be used to key
/// a resource list. Keep in mind that the ID returned
/// must always be **unique** for any given instance!
pub trait ResourceId {
    fn resource_id(&self) -> usize;
}

/// A generic resource manager for any type of resource.
pub struct ResourceManager<T> {
    list: HashMap<usize, T>,
}

impl<T: Clone> ResourceManager<T> {
    /// Create a new `ResourceManager` with no loaded resources.
    pub fn new() -> ResourceManager<T> {
        ResourceManager {
            list: HashMap::new(),
        }
    }

    /// Add a resource with an ID. **If the resource already exists,
    /// it will be replaced**.
    /// Returns `true` if the resource was replaced.
    pub fn add<I: ResourceId>(&mut self, idx: I, res: T) -> bool {
        self.list.insert(idx.resource_id(), res).is_some()
    }

    /// Returns the resource with the specified ID, if it exists.
    /// If the resource is not found, `None` is returned.
    pub fn get<I: ResourceId>(&self, idx: I) -> Option<T> {
        if let Some(r) = self.list.get(&idx.resource_id()) {
            Some(r.clone())
        } else {
            None
        }
    }
}
