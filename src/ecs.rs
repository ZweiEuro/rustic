use std::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    components::{Collision, Drawable, Physics},
    gen_vec::GenVec,
};

type EntityVec<T> = GenVec<Option<T>>;

struct ECS {
    physics_components: RwLock<EntityVec<Physics>>,
    collision_components: EntityVec<Collision>,
    drawable_components: EntityVec<Drawable>,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            physics_components: RwLock::new(GenVec::new()),
            collision_components: GenVec::new(),
            drawable_components: GenVec::new(),
        }
    }

    // get a read-only copy of the component
    fn get_physics_component<T>(&self) -> RwLockReadGuard<'_, GenVec<Option<Physics>>> {
        self.physics_components.read().unwrap()
    }

    fn get_physics_component_mut<T>(&self) -> RwLockWriteGuard<'_, GenVec<Option<Physics>>> {
        self.physics_components.write().unwrap()
    }
}
