pub mod menu_state;
pub mod play_state;

use settings::Settings;
use specs::World;
use std::collections::HashMap;

use components::StateChange;
use conrod::Ui;

pub trait State {
    fn setup(&mut self, world: &mut World);
    fn update(&mut self, &mut World);
    fn handle_custom_change(&mut self, &String, &mut World);
    fn get_ui_to_render(&mut self) -> Option<&mut Ui>;
    fn should_render_ui(&self) -> bool;
    fn create_ui_widgets(&mut self, settings: &mut Settings) -> Option<String>;
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

    pub fn cleanup_state(&self, world: &mut World) {
        world.delete_all();
    }

    pub fn process_state_change(&mut self, state_change: &mut StateChange, world: &mut World) {
        if state_change.action != "" && state_change.state != "" {
            if state_change.action == "restart" {
                self.restart_current_state(world);
            } else if state_change.action == "start" {
                self.swap_state(state_change.state.clone(), world);
            } else {
                self.states
                    .get_mut(&self.current_state)
                    .unwrap()
                    .handle_custom_change(&state_change.action, world);
            }
        }
    }

    pub fn restart_current_state(&mut self, world: &mut World) {
        self.cleanup_state(world);
        if let Some(current_state) = self.states.get_mut(&self.current_state) {
            current_state.setup(world);
            world.maintain();
        }
    }

    pub fn swap_state(&mut self, name: String, world: &mut World) {
        self.cleanup_state(world);
        self.current_state = name;
        self.states
            .get_mut(&self.current_state)
            .unwrap()
            .setup(world);

        world.maintain();
    }

    pub fn get_ui_to_render(&mut self) -> Option<&mut Ui> {
        self.states
            .get_mut(&self.current_state)
            .unwrap()
            .get_ui_to_render()
    }

    pub fn should_render_ui(&self) -> bool {
        self.states
            .get(&self.current_state)
            .unwrap()
            .should_render_ui()
    }

    pub fn create_ui_widgets(&mut self, settings: &mut Settings) -> Option<String> {
        self.states
            .get_mut(&self.current_state)
            .unwrap()
            .create_ui_widgets(settings)
    }

    pub fn update(&mut self, world: &mut World) {
        self.states
            .get_mut(&self.current_state)
            .unwrap()
            .update(world);
    }
}
