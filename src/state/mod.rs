pub mod play_state;

use std::sync::{Arc, Mutex};

use specs::World;
use std::collections::HashMap;

use scene::Scene;
use scene::node::Node;

pub trait State {
    fn setup(&mut self, world: &mut World) {}
    fn get_scene(&self) -> Arc<Mutex<Scene>>;
}

pub struct StateManager {
    current_state: String,
    states: HashMap<String, Box<State>>,
}

impl StateManager {
    pub fn new() -> StateManager {
        let mut states: HashMap<String, Box<State>> = HashMap::new();
        StateManager{
            current_state: "".to_string(),
            states: HashMap::new(),
        }
    }

    pub fn add_state(&mut self, name: String, state: Box<State>) {
        self.states.insert(name, state);
    }

    fn cleanup_state (&self, state: &Box<State>, world: &mut World) {
        let scene = state.get_scene();
        let scene = scene.lock().unwrap();
        for node in &scene.nodes {
            self.delete_entities_from_node(node, world);
        }
    }

    fn delete_entities_from_node(&self, node: &Node, world: &mut World) {
        if let Some(entity) = node.entity {
            world.delete_entity(entity);
        }

        for node in &node.sub_nodes {
            self.delete_entities_from_node(node, world);
        }
    }

    pub fn get_current_scene(&self) -> Arc<Mutex<Scene>> {
        self.states.get(&self.current_state).unwrap().get_scene()
    }

    pub fn restart_current_state(&mut self, world: &mut World) {
        // separate if blocks to have different mutable/immutable borrows
        if let Some(current_state) = self.states.get(&self.current_state) {
            self.cleanup_state(current_state, world);
        }

        if let Some(current_state) = self.states.get_mut(&self.current_state) {
            current_state.setup(world);
        }
    }

    pub fn swap_state(&mut self, name: String, world: &mut World) {
        if let Some(current_state) = self.states.get(&self.current_state) {
            self.cleanup_state(current_state, world);
        }

        self.current_state = name;
        self.states.get_mut(&self.current_state).unwrap().setup(world);
    }
}