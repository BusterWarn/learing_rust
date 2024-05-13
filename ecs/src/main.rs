const DEBUG: bool = false;

// Define component masks with each bit representing a different component
const COMPONENT_POSITION: u32 = 0b0001; // Binary 0001
const COMPONENT_VELOCITY: u32 = 0b0010; // Binary 0010
                                        // Add more components with unique bit positions
                                        // const COMPONENT_HEALTH: u32 = 0b0100; // Binary 0100

#[derive(Copy, Clone, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone)]
struct Direction {
    x: f32,
    y: f32,
}

#[derive(Copy, Clone)]
struct Velocity {
    speed: f32,
}

fn calculate_desired_speed(from: &Position, to: &Position, max_velocity: Velocity) -> Velocity {
    let distance_to_target = ((to.x - from.x).powi(2) + (to.y - from.y).powi(2)).sqrt();

    if distance_to_target < max_velocity.speed {
        Velocity {
            speed: distance_to_target,
        }
    } else {
        max_velocity
    }
}

fn calculate_move_vector(from: &Position, to: &Position) -> Direction {
    Direction {
        x: to.x - from.x,
        y: to.y - from.y,
    }
}

fn update_position(position: &mut Position, move_vector: &Direction, velocity: &Velocity) {
    // Normalize the move vector
    let magnitude = (move_vector.x.powi(2) + move_vector.y.powi(2)).sqrt();
    let normalized_move_vector = if magnitude != 0.0 {
        Position {
            x: move_vector.x / magnitude,
            y: move_vector.y / magnitude,
        }
    } else {
        // If magnitude is 0, return to avoid division by zero
        return;
    };

    if DEBUG {
        print!("Position before: {:?}", position);
    }

    // Update position based on normalized direction and velocity
    position.x += normalized_move_vector.x * velocity.speed;
    position.y += normalized_move_vector.y * velocity.speed;

    if DEBUG {
        println!(" - after: {:?}", position);
    }
}

const SOUTH_POLE: Position = Position { x: 0.0, y: 0.0 };

struct World {
    entities: Vec<u32>,
    positions: Vec<Option<Position>>,
    velocities: Vec<Option<Velocity>>,
    component_masks: Vec<u32>,
    next_entity_id: u32,
}

impl World {
    fn new() -> Self {
        World {
            entities: vec![],
            positions: vec![],
            velocities: vec![],
            component_masks: vec![],
            next_entity_id: 0,
        }
    }

    fn create_entity(&mut self) -> u32 {
        let id = self.next_entity_id;
        self.entities.push(id);
        self.positions.push(None);
        self.velocities.push(None);
        self.component_masks.push(0);
        self.next_entity_id += 1;
        id
    }

    // Add a position component to an entity
    fn add_position(&mut self, entity_id: u32, position: Position) {
        let index = self
            .entities
            .iter()
            .position(|&id| id == entity_id)
            .unwrap();
        self.positions[index] = Some(position);
        self.component_masks[index] |= COMPONENT_POSITION; // Set the position bit
    }

    // Add a velocity component to an entity
    fn add_velocity(&mut self, entity_id: u32, velocity: Velocity) {
        let index = self
            .entities
            .iter()
            .position(|&id| id == entity_id)
            .unwrap();
        self.velocities[index] = Some(velocity);
        self.component_masks[index] |= COMPONENT_VELOCITY; // Set the velocity bit
    }

    // Example of removing a component
    fn remove_position(&mut self, entity_id: u32) {
        let index = self
            .entities
            .iter()
            .position(|&id| id == entity_id)
            .unwrap();
        self.positions[index] = None;
        self.component_masks[index] &= !COMPONENT_POSITION; // Clear the position bit
    }

    // Check if an entity has all the specified components
    fn has_components(&self, entity_id: u32, components_mask: u32) -> bool {
        let index = self
            .entities
            .iter()
            .position(|&id| id == entity_id)
            .unwrap();
        (self.component_masks[index] & components_mask) == components_mask
    }

    // Add a component to an entity
    fn add_component_mask(&mut self, entity_id: u32, component_mask: u32) {
        let index = self
            .entities
            .iter()
            .position(|&id| id == entity_id)
            .unwrap();
        self.component_masks[index] |= component_mask;
    }

    // Remove a component from an entity
    fn remove_component_mask(&mut self, entity_id: u32, component_mask: u32) {
        let index = self
            .entities
            .iter()
            .position(|&id| id == entity_id)
            .unwrap();
        self.component_masks[index] &= !component_mask;
    }

    fn update(&mut self) {
        let mut updates = Vec::new();

        // First, collect the updates to avoid borrowing conflicts
        for (index, entity) in self.entities.iter().enumerate() {
            // Check if the entity has both Position and Velocity components
            if self.has_components(*entity, COMPONENT_POSITION | COMPONENT_VELOCITY) {
                // Safely unwrap positions and velocities because we know they exist
                if let (Some(pos), Some(vel)) = (&self.positions[index], &self.velocities[index]) {
                    let desired_velocity = calculate_desired_speed(pos, &SOUTH_POLE, *vel);
                    let move_vector = calculate_move_vector(pos, &SOUTH_POLE);
                    updates.push((*entity, move_vector, desired_velocity));
                }
            }
        }

        // Apply the updates
        for (entity, move_vector, velocity) in updates {
            let index = self.entities.iter().position(|&id| id == entity).unwrap();
            if let Some(pos) = &mut self.positions[index] {
                update_position(pos, &move_vector, &velocity);

                // After updating the position, check if it's not at the SOUTH POLE
                if pos.x != 0.0 || pos.y != 0.0 {
                    println!("Goose {} at {:?} honks: \"Honk!\"", entity, pos);
                }
            }
        }
    }
}

fn main() {
    let mut world = World::new();
    const NUM_GEESE: u32 = 5;

    // Create entities and add components using bitmask functions
    for _ in 0..NUM_GEESE {
        let entity = world.create_entity();
        let x = rand::random::<f32>() * 10.0; // Generate random x coordinate
        let y = rand::random::<f32>() * 10.0; // Generate random y coordinate

        // Add position and velocity components
        world.add_position(entity, Position { x, y });
        world.add_velocity(entity, Velocity { speed: 1.0 });

        // Set the component masks for position and velocity
        world.add_component_mask(entity, COMPONENT_POSITION);
        world.add_component_mask(entity, COMPONENT_VELOCITY);
    }

    // Update the world a few times to simulate movement
    for tick in 0..15 {
        println!("Tick {} of the world", tick + 1);
        world.update();
    }
}
