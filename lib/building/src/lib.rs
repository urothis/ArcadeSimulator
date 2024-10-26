pub mod event;

use bevy::{prelude::*, utils::HashMap};
use prelude::*;

pub mod prelude {
    pub use crate::*;
    pub use crate::event::floor::*;
}

pub type Floors = HashMap<i8, Floor>;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Reflect)]
pub struct SimulatorBuildingPlugin;

#[derive(Debug, Default, PartialEq, Eq, Clone, Reflect, Resource)]
pub struct SimulatorBuildingState {
    pub floors: Floors,
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Reflect)]
pub struct Floor {
    pub depth_slots_unlocked: u8,
    pub width_slots_unlocked: u8,
}

impl Floor {
    pub fn default_ground_floor() -> Self {
        Self {
            depth_slots_unlocked: 5,
            width_slots_unlocked: 10
        }
    }

    pub fn default_basement() -> Self {
        Self {
            depth_slots_unlocked: 5,
            width_slots_unlocked: 10
        }
    }
}

impl Plugin for SimulatorBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulatorBuildingState>();
        app.add_event::<FloorEvent>();
        app.add_systems(Startup, startup);
        app.add_systems(Update, update);
    }
}

fn startup(
    mut _commands: Commands,
    mut state: ResMut<SimulatorBuildingState>,
) {
    // ground floor
    state.floors.insert(0, Floor::default_ground_floor());
    // basement
    state.floors.insert(-1, Floor::default_basement());
}

fn update(
    mut _commands: Commands,
    mut state: ResMut<SimulatorBuildingState>,
    mut events: EventReader<FloorEvent>,
) {
    for event in events.read() {
        match event {
            FloorEvent::New(sign, floor_offset) => {
                let _floor = match sign {
                    Sign::Positive => {
                        state.floors.insert(*floor_offset, Floor::default_ground_floor());
                    },
                    Sign::Negative => {
                        state.floors.insert(*floor_offset, Floor::default_basement());
                    },
                };
            },
            FloorEvent::Customize(floor_index, customize_floor_event) => {
                let _floor = state.floors.get_mut(floor_index).unwrap();
                match customize_floor_event {
                    CustomizeFloorEvent::Ceiling => {
                        // customize the ceiling
                    },
                    CustomizeFloorEvent::Floor => {
                        // customize the floor
                    },
                    CustomizeFloorEvent::Wall => {
                        // customize the wall
                    },
                }
            },
            FloorEvent::Resize(floor_index, depth_slots, width_slots) => {
                let floor = state.floors.get_mut(floor_index).unwrap();
                floor.depth_slots_unlocked = *depth_slots;
                floor.width_slots_unlocked = *width_slots;
            },
            FloorEvent::Remove(floor_index) => {
                state.floors.remove(floor_index);
            },
        }
    }
}

