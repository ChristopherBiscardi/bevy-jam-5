use bevy::prelude::*;
use ulid::Ulid;

pub struct PersistentIdPlugin;

impl Plugin for PersistentIdPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PersistentId>();
    }
}

// TODO: Inventory must be associated with
// a customer. Otherwise multiple customers
// is buggy because returning items goes to arbitrary
// customer
#[derive(
    Debug, Component, Reflect, Clone, PartialEq, Eq,
)]
#[reflect(Component)]
pub struct PersistentId {
    id: Id,
}

#[derive(Debug, Reflect, Clone, Eq, PartialEq)]
struct Id(String);

// impl FromReflect for Ulid {
//     fn from_reflect(reflect: &dyn Reflect) -> Option<Self> {
//         todo!()
//     }
// }

impl PersistentId {
    pub fn new() -> Self {
        let ulid = Ulid::new();
        PersistentId {
            id: Id(ulid.to_string()),
        }
    }
}
