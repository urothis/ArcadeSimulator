use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub struct Floor {
    pub depth_slots_unlocked: u8,
    pub width_slots_unlocked: u8,
}

impl Default for Floor {
    fn default() -> Self {
        Self {
            depth_slots_unlocked: 5,
            width_slots_unlocked: 10,
        }
    }
}
