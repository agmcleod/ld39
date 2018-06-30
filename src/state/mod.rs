pub mod play_state;

use std::sync::{Arc, Mutex};

use specs::World;
use std::collections::HashMap;

use scene::Node;
use conrod::Ui;
use components::StateChange;

pub trait State {
    fn setup(&mut self, world: &mut World);
    fn get_scene(&self) -> Arc<Mutex<Node>>;
    fn update(&mut self, &mut World);
    fn handle_custom_change(&mut self, &String);
    fn get_ui_to_render(&mut self) -> Option<&Ui>;
}

pub struct StateManager {
    current_state: String,
    states: HashMap<String, Box<State>>,
    pub restart_next_frame: bool,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager {
            current_state: "".to_string(),
            states: HashMap::new(),
            restart_next_frame: false,
        }
    }

    pub fn add_state(&mut self, name: String, state: Box<State>) {
        self.states.insert(name, state);
    }

    fn cleanup_state(&self, state: &Box<State>, world: &mut World) {
        let scene = state.get_scene();
        let scene = scene.lock().unwrap();
        for node in scene.get_sub_nodes() {
            self.delete_entities_from_node(node, world);
        }
    }

    fn delete_entities_from_node(&self, node: &Node, world: &mut World) {
        if let Some(entity) = node.entity {
            world.delete_entity(entity).unwrap();
        }

        for node in node.get_sub_nodes() {
            self.delete_entities_from_node(node, world);
        }
    }

    pub fn get_current_scene(&self) -> Arc<Mutex<Node>> {
        self.states.get(&self.current_state).unwrap().get_scene()
    }

    pub fn process_state_change(&mut self, state_change: &mut StateChange, world: &mut World) {
        if state_change.action != "" && state_change.state != "" {
            if state_change.action == "restart" {
                self.restart_current_state(world);
            } else {
                self.states
                    .get_mut(&self.current_state)
                    .unwrap()
                    .handle_custom_change(&state_change.action);
            }
        }
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
        self.states
            .get_mut(&self.current_state)
            .unwrap()
            .setup(world);
    }

    pub fn get_ui_to_render(&mut self) -> Option<&Ui> {
        self.states
            .get_mut(&self.current_state)
            .unwrap()
            .get_ui_to_render()
    }

    pub fn update(&mut self, world: &mut World) {
        self.states
            .get_mut(&self.current_state)
            .unwrap()
            .update(world);
    }
}
