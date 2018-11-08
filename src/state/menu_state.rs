use conrod::Ui;
use rand::{thread_rng};
use specs::{Dispatcher, DispatcherBuilder, World};

use components::{Button, Color, EntityLookup, MenuScreen, Node, Sprite, Texture, Transform};
use settings::Settings;
use state::State;
use systems;

pub struct MenuState<'a> {
    dispatcher: Dispatcher<'a, 'a>,
    screen_sizes: [(u16, u16); 4],
}

impl<'a> MenuState<'a> {
    pub fn new(screen_sizes: [(u16, u16); 4]) -> Self {
        let dispatcher = DispatcherBuilder::new()
            .with(systems::ButtonHover {}, "button_hover", &[])
            .with(systems::TextAbsoluteCache {}, "text_absolute_cache", &[])
            .with(systems::MenuAnimation::new(), "menu_animation", &[])
            .build();

        MenuState {
            dispatcher,
            screen_sizes,
        }
    }

    pub fn get_name() -> String {
        "menu_state".to_string()
    }
}

impl<'a> State for MenuState<'a> {
    fn setup(&mut self, world: &mut World) {
        let mut rng = thread_rng();
        let pos = MenuScreen::get_random_position(&mut rng);
        let end_pos = MenuScreen::get_random_position(&mut rng);
        let menu_screen = MenuScreen::new(0.0, pos.clone(), end_pos);

        let image = world
            .create_entity()
            .with(Transform::visible(
                pos.0,
                pos.1,
                4.0,
                self.screen_sizes[0].0,
                self.screen_sizes[0].1,
                0.0,
                2.0,
                2.0,
            ))
            .with(menu_screen)
            .with(Texture::new("screenone.png"))
            .with(Color([1.0, 1.0, 1.0, 1.0]))
            .build();

        let mut child_entities = vec![image];

        let pos = MenuScreen::get_random_position(&mut rng);
        let end_pos = MenuScreen::get_random_position(&mut rng);
        let menu_screen = MenuScreen::new(2.0, pos.clone(), end_pos);

        let image = world
            .create_entity()
            .with(Transform::new(
                pos.0,
                pos.1,
                3.0,
                self.screen_sizes[0].0,
                self.screen_sizes[0].1,
                0.0,
                2.0,
                2.0,
                false,
            ))
            .with(menu_screen)
            .with(Texture::new("screentwo.png"))
            .with(Color([1.0, 1.0, 1.0, 1.0]))
            .build();
        child_entities.push(image);

        let pos = MenuScreen::get_random_position(&mut rng);
        let end_pos = MenuScreen::get_random_position(&mut rng);
        let menu_screen = MenuScreen::new(4.0, pos.clone(), end_pos);

        let image = world
            .create_entity()
            .with(Transform::new(
                pos.0,
                pos.1,
                2.0,
                self.screen_sizes[0].0,
                self.screen_sizes[0].1,
                0.0,
                2.0,
                2.0,
                false,
            ))
            .with(menu_screen)
            .with(Texture::new("screenthree.png"))
            .with(Color([1.0, 1.0, 1.0, 1.0]))
            .build();
        child_entities.push(image);

        let pos = MenuScreen::get_random_position(&mut rng);
        let end_pos = MenuScreen::get_random_position(&mut rng);
        let menu_screen = MenuScreen::new(6.0, pos.clone(), end_pos);

        let image = world
            .create_entity()
            .with(Transform::new(
                pos.0,
                pos.1,
                1.0,
                self.screen_sizes[0].0,
                self.screen_sizes[0].1,
                0.0,
                2.0,
                2.0,
                false,
            ))
            .with(menu_screen)
            .with(Texture::new("screenfour.png"))
            .with(Color([1.0, 1.0, 1.0, 1.0]))
            .build();
        child_entities.push(image);

        let entity = world.create_entity()
            .with(Sprite{ frame_name: "title.png".to_string() })
            .with(Transform::visible(
                112.0,
                50.0,
                5.0,
                735,
                228,
                0.0,
                1.0,
                1.0
            ))
            .build();

        child_entities.push(entity);

        let entity = world.create_entity()
            .with(Sprite{ frame_name: "start.png".to_string() })
            .with(Button::new("start".to_string(), ["start.png".to_string(), "start_hover.png".to_string()]))
            .with(Transform::visible(
                384.0,
                450.0,
                5.0,
                192,
                50,
                0.0,
                1.0,
                1.0
            ))
            .build();

        child_entities.push(entity);

        let mut lookup = EntityLookup::new();

        let mut root = Node::new();
        root.add_many(child_entities);

        let root_entity = world.create_entity().with(root).build();
        lookup.entities.insert("root".to_string(), root_entity);

        world.add_resource(lookup);
    }

    fn update(&mut self, world: &mut World) {
        self.dispatcher.dispatch(&mut world.res);
    }

    fn handle_custom_change(&mut self, _: &String, _: &mut World) {}

    fn get_ui_to_render(&mut self) -> Option<&mut Ui> {
        None
    }

    fn create_ui_widgets(&mut self, _: &mut Settings) -> Option<String> {
        None
    }

    fn should_render_ui(&self) -> bool {
        false
    }
}
