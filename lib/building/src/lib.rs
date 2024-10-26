pub mod event;
pub mod floor;

use bevy::{prelude::*, utils::HashMap};
use prelude::*;

pub mod prelude {
    pub use crate::event::floor::*;
    pub use crate::floor::*;
    pub use crate::*;
}

/// FloorOffset is the distance from the ground floor, 0 is the ground floor, -1 is the first, basement.
pub type FloorOffset = i8;
/// FloorDepth is the number of slots in the depth of the floor.
pub type FloorDepthSlots = u8;
/// FloorWidth is the number of slots in the width of the floor.
pub type FloorWidthSlots = u8;
/// usage should the floor index, 0 is the ground floor, -1 is the first, basement.
pub type FloorIndex = i8;

#[derive(Debug, PartialEq, Reflect)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Reflect)]
pub enum Dimension {
    Depth,
    Width,
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Reflect)]
pub struct SimulatorBuildingPlugin;

#[derive(Debug, PartialEq, Eq, Clone, Reflect, Resource)]
pub struct SimulatorBuildingState {
    pub floors: HashMap<i8, Floor>,
}

// default state for the building
impl Default for SimulatorBuildingState {
    fn default() -> Self {
        let mut floors = HashMap::default();
        floors.insert(0, Floor::default());
        floors.insert(-1, Floor::default());

        Self { floors }
    }
}

impl Plugin for SimulatorBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulatorBuildingState>();
        app.add_event::<FloorEvent>();
        app.add_systems(Startup, startup);
        app.add_systems(Update, handle_floor_events);
    }
}

fn startup(mut _commands: Commands) {}
