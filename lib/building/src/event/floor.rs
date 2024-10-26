use bevy::prelude::*;

use crate::prelude::*;

#[derive(Debug, Event, Reflect)]
pub enum FloorEvent {
    /// Add a new floor, the tuple contains the sign and the floor offset.
    New(Sign),
    /// Choose between the ceiling, the floor, or the wall.
    /// And then choose the material.
    Customize(FloorIndex, CustomizeFloorEvent),
    /// Expand the floor, the tuple contains the sign, the floor index, and the dimension to expand.
    Expand(FloorIndex, Dimension),
    /// Resize the floor to an exact size.
    Resize(FloorIndex, FloorDepthSlots, FloorWidthSlots),
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

pub fn handle_floor_events(
    mut _commands: Commands,
    mut state: ResMut<SimulatorBuildingState>,
    mut events: EventReader<FloorEvent>,
) {
    for event in events.read() {
        match event {
            FloorEvent::New(sign) => {
                match sign {
                    // if positive, add a floor to the very top
                    Sign::Positive => {
                        println!("Floors: {:?}", state.floors);

                        let floor_index = state.floors.keys().max().unwrap() + 1;
                        println!("Adding a new floor at index: {}", floor_index);
                        state.floors.insert(floor_index, Floor::default());
                        println!("Floors: {:?}", state.floors);
                    }
                    // if negative, add a floor to the very bottom of the hashmap
                    Sign::Negative => {
                        let floor_index = state.floors.keys().min().unwrap() - 1;
                        println!("Adding a new floor at index: {}", floor_index);
                        state.floors.insert(floor_index, Floor::default());
                        println!("Floors: {:?}", state.floors);
                    }
                };
            }
            FloorEvent::Customize(floor_index, customize_floor_event) => {
                let _floor = state.floors.get_mut(floor_index).unwrap();
                match customize_floor_event {
                    CustomizeFloorEvent::Ceiling => {
                        // customize the ceiling
                    }
                    CustomizeFloorEvent::Floor => {
                        // customize the floor
                    }
                    CustomizeFloorEvent::Wall => {
                        // customize the wall
                    }
                }
            }
            FloorEvent::Expand(floor_index, socket_to_expand) => {
                let floor = state.floors.get_mut(floor_index).unwrap();
                match socket_to_expand {
                    Dimension::Depth => {
                        floor.depth_slots_unlocked += 1;
                    }
                    Dimension::Width => {
                        floor.width_slots_unlocked += 1;
                    }
                }
            }
            FloorEvent::Resize(floor_index, depth_slots, width_slots) => {
                let floor = state.floors.get_mut(floor_index).unwrap();
                floor.depth_slots_unlocked = *depth_slots;
                floor.width_slots_unlocked = *width_slots;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shim() -> App {
        let mut app = App::new();
        app.insert_resource(SimulatorBuildingState::default());
        app.add_event::<FloorEvent>();
        app.add_systems(Startup, startup);
        app.add_systems(Update, handle_floor_events);
        app
    }

    #[test]
    fn test_event_new_positive_floor() {
        let mut app = shim();
        // Send an `FloorEvent::New` event
        app.world_mut()
            .resource_mut::<Events<FloorEvent>>()
            .send(FloorEvent::New(Sign::Positive));
        // Run systems
        app.update();
        // Check resulting changes
        assert_eq!(app.world().resource::<SimulatorBuildingState>().floors, {
            let mut floors = HashMap::default();
            floors.insert(-1, Floor::default());
            floors.insert(0, Floor::default());
            floors.insert(1, Floor::default());
            floors
        });
    }

    #[test]
    fn test_event_new_negative_floor() {
        let mut app = shim();
        // Send an `EnemyDied` event
        app.world_mut()
            .resource_mut::<Events<FloorEvent>>()
            .send(FloorEvent::New(Sign::Negative));
        // Run systems
        app.update();
        // Check resulting changes
        assert_eq!(app.world().resource::<SimulatorBuildingState>().floors, {
            let mut floors = HashMap::default();
            floors.insert(-1, Floor::default());
            floors.insert(0, Floor::default());
            floors.insert(-2, Floor::default());
            floors
        });
    }

    #[test]
    fn test_event_expand_depth_floor() {
        let mut app = shim();
        // Send an `EnemyDied` event
        app.world_mut()
            .resource_mut::<Events<FloorEvent>>()
            .send(FloorEvent::Expand(0, Dimension::Depth));
        // Run systems
        app.update();
        // Check resulting changes
        assert_eq!(
            app.world().resource::<SimulatorBuildingState>().floors[&0],
            Floor {
                depth_slots_unlocked: 6,
                ..default()
            }
        );
    }

    #[test]
    fn test_event_expand_width_floor() {
        let mut app = shim();
        // Send an `EnemyDied` event
        app.world_mut()
            .resource_mut::<Events<FloorEvent>>()
            .send(FloorEvent::Expand(0, Dimension::Width));
        // Run systems
        app.update();
        // Check resulting changes
        assert_eq!(
            app.world().resource::<SimulatorBuildingState>().floors[&0],
            Floor {
                width_slots_unlocked: 11,
                ..default()
            }
        );
    }
}
