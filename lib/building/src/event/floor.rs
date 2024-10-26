use bevy::prelude::*;

/// I need to figure out the proper std lib type to use for this.
#[derive(Debug, PartialEq, Reflect)]
pub enum Sign {
    Positive,
    Negative,
}

/// FloorOffset is the distance from the ground floor, 0 is the ground floor, -1 is the first, basement.
pub type FloorOffset = i8;
/// FloorDepth is the number of slots in the depth of the floor.
pub type FloorDepthSlots = u8;
/// FloorWidth is the number of slots in the width of the floor.
pub type FloorWidthSlots = u8;
/// usage should the floor index, 0 is the ground floor, -1 is the first, basement.
pub type FloorIndex = i8;

#[derive(Debug, Event, Reflect)]
pub enum FloorEvent {
    /// Add a new floor, the tuple contains the sign and the floor offset.
    New(Sign, FloorOffset),
    /// Choose between the ceiling, the floor, or the wall.
    /// And then choose the material.
    Customize(FloorIndex, CustomizeFloorEvent),
    /// Resize the floor, the tuple contains the sign, the floor index, the depth slots, and the width slots.
    Resize(FloorIndex, FloorDepthSlots, FloorDepthSlots),
    /// Remove the floor, from the floor index.
    /// This will reshift the floor HashMap to be contiguous.
    Remove(FloorIndex),
}

#[derive(Debug, Event, Reflect)]
pub enum CustomizeFloorEvent {
    /// Customize the ceiling.
    Ceiling,
    /// Customize the floor.
    Floor,
    /// Customize the wall.
    Wall,
}
