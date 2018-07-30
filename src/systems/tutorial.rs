use components::{ui::TutorialUI, Actions, Color, DeltaTime, EntityLookup, Node, Pulse, Rect,
                 Shape, Text, Tile, TileNodes, TileType, Transform, TutorialStep,
                 CITY_POWER_STATE_COORDS};
use entities::tutorial;
use renderer;
use settings::Settings;
use specs::{Entities, Join, Read, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};

struct StepCreationDetails<'a> {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    message: &'a str,
}

impl<'a> StepCreationDetails<'a> {
    fn new(x: f32, y: f32, w: f32, h: f32, message: &'a str) -> Self {
        StepCreationDetails {
            x,
            y,
            w,
            h,
            message,
        }
    }
}

pub struct Tutorial {
    hide_last_step_time: f32,
}

impl Tutorial {
    pub fn new() -> Self {
        Tutorial {
            hide_last_step_time: 0.0,
        }
    }
}

impl<'a> System<'a> for Tutorial {
    type SystemData = (
        Entities<'a>,
        Read<'a, Actions>,
        WriteStorage<'a, Color>,
        Read<'a, DeltaTime>,
        Read<'a, EntityLookup>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Pulse>,
        WriteStorage<'a, Rect>,
        Write<'a, Settings>,
        WriteStorage<'a, Shape>,
        WriteStorage<'a, Text>,
        Read<'a, TileNodes>,
        WriteStorage<'a, Transform>,
        Write<'a, TutorialStep>,
        WriteStorage<'a, TutorialUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            actions_storage,
            mut color_storage,
            delta_time_storage,
            entity_lookup_storage,
            mut node_storage,
            mut pulse_storage,
            mut rect_storage,
            mut settings_storage,
            mut shape_storage,
            mut text_storage,
            tile_nodes_storage,
            mut transform_storage,
            mut tutorial_step_storage,
            mut tutorial_ui_storage,
        ) = data;

        // TOOD: return out from this function if settings has tutorial completed as true

        let actions = actions_storage.deref();

        let mut details = None;

        if actions.action_fired(&TutorialStep::SelectTile.as_string()) {
            let tile_nodes = tile_nodes_storage.deref();

            let mut target_cell = (0.0, 0.0);

            for x in 0i32..10i32 {
                for y in 0i32..10i32 {
                    if let Some((tile_type, _)) = tile_nodes.nodes.get(&(x, y)) {
                        if *tile_type == TileType::Open {
                            let mut safe_space = true;
                            'check_neighbours: for x2 in -1..2 {
                                for y2 in -1..2 {
                                    if x2 == 0 && y2 == 0 {
                                        continue;
                                    }
                                    if let Some((tile_type, _)) =
                                        tile_nodes.nodes.get(&(x2 + x, y2 + y))
                                    {
                                        if *tile_type != TileType::Open {
                                            safe_space = false;
                                            break 'check_neighbours;
                                        }
                                    }
                                }
                            }

                            if safe_space {
                                target_cell.0 = x as f32;
                                target_cell.1 = y as f32;
                            }
                        }
                    }
                }
            }

            let size = Tile::get_size();
            target_cell.0 *= size;
            target_cell.1 *= size;

            details = Some(StepCreationDetails::new(
                target_cell.0,
                target_cell.1,
                size,
                size,
                "To start collecting resources, click the glowing tile",
            ));
        } else if actions.action_fired(&TutorialStep::BuildCoal(0.0, 0.0).as_string()) {
            let tutorial_step = tutorial_step_storage.deref();
            match *tutorial_step {
                TutorialStep::BuildCoal(x, y) => {
                    details = Some(StepCreationDetails::new(
                        x + 10.0,
                        y + 10.0,
                        64.0,
                        64.0,
                        "Click the icon here to build a coal mine operation",
                    ));
                }
                _ => {}
            }
        } else if actions.action_fired(&TutorialStep::CoalGathered.as_string()) {
            details = Some(StepCreationDetails::new(
                670.0,
                158.0,
                100.0,
                40.0,
                "After building the coal mine, you are now collecting coal as a resource.",
            ));
        } else if actions.action_fired(&TutorialStep::SellResources.as_string()) {
            details = Some(StepCreationDetails::new(
                822.0,
                576.0,
                96.0,
                32.0,
                "Click the button at the bottom right to sell",
            ));
        } else if actions.action_fired(&TutorialStep::ResourcesSold.as_string()) {
            details = Some(StepCreationDetails::new(
                CITY_POWER_STATE_COORDS[0].0 + 640.0,
                CITY_POWER_STATE_COORDS[0].1,
                280.0,
                360.0,
                "When you sell, the resources go to the power grid filling up the city power bars. Your money also goes up.\n\nUse money to keep building. Be wary of building next to a tile occupied by a city or by nature."
            ));
        } else if actions.action_fired(&TutorialStep::ShowUpgrades.as_string()) {
            details = Some(StepCreationDetails::new(
                683.0,
                576.0,
                96.0,
                32.0,
                "Click the button to the bottom right to view available upgrades. Don't worry, this button pauses the game."
            ));
        } else if actions.action_fired(&TutorialStep::Upgrade.as_string()) {
            let dimensions = renderer::get_dimensions();
            let width = dimensions[0] - 640.0;
            details = Some(StepCreationDetails::new(
                640.0 + width * 0.75 - 16.0,
                96.0,
                32.0,
                32.0,
                "Research this upgrade to collect more resources when one mine, or other resource collection facilities of the same type are adjacent."
            ));
        } else if actions.action_fired(&TutorialStep::Resume.as_string()) {
            details = Some(StepCreationDetails::new(
                752.0,
                576.0,
                96.0,
                32.0,
                "Resume the game to continue playing, and let the upgrade research.",
            ));
        } else if actions.action_fired(&TutorialStep::Objective(0.0).as_string()) {
            self.hide_last_step_time = 10.0;
            details = Some(StepCreationDetails::new(
                CITY_POWER_STATE_COORDS[0].0 + 640.0,
                CITY_POWER_STATE_COORDS[0].1,
                280.0,
                100.0,
                "When the power # becomes positive, it means you are producing enough to supply this city. Click Power Additional City to add another city. Having a positive output with all 4 is the goal of the game.\n\nBest of luck!"
            ));
        }

        let tutorial_step = tutorial_step_storage.deref_mut();
        match *tutorial_step {
            TutorialStep::Objective(time_left) => {
                if time_left > 0.0 {
                    *tutorial_step =
                        TutorialStep::Objective(time_left - delta_time_storage.deref().dt);
                    if time_left - delta_time_storage.deref().dt <= 0.0 {
                        for (entity, _) in (&*entities, &tutorial_ui_storage).join() {
                            entities.delete(entity).unwrap();
                        }
                        let settings = settings_storage.deref_mut();
                        settings.set_completed_tutorial(true);
                    }
                }
            }
            _ => {}
        }

        if let Some(details) = details {
            tutorial::create_step(
                &entities,
                &mut color_storage,
                &entity_lookup_storage,
                &mut node_storage,
                &mut pulse_storage,
                &mut rect_storage,
                &mut shape_storage,
                &mut text_storage,
                &mut tutorial_ui_storage,
                &mut transform_storage,
                details.x,
                details.y,
                details.w,
                details.h,
                details.message,
            );
        }
    }
}
