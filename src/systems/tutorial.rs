use components::{Actions, Color, EntityLookup, Node, Rect, Pulse, Shape, Text, Tile, TileNodes, TileType, Transform, TutorialStep, ui::{TutorialUI}};
use specs::{Entities, Read, System, WriteStorage};
use std::ops::Deref;
use entities::tutorial;

struct StepCreationDetails<'a> {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    message: &'a str
}

impl <'a>StepCreationDetails<'a> {
    fn new(x: f32, y: f32, w: f32, h: f32, message: &'a str) -> Self {
        StepCreationDetails{
            x, y, w, h, message
        }
    }
}

pub struct Tutorial;

impl<'a> System<'a> for Tutorial {
    type SystemData = (
        Entities<'a>,
        Read<'a, Actions>,
        WriteStorage<'a, Color>,
        Read<'a, EntityLookup>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Pulse>,
        WriteStorage<'a, Rect>,
        WriteStorage<'a, Shape>,
        WriteStorage<'a, Text>,
        Read<'a, TileNodes>,
        WriteStorage<'a, Transform>,
        Read<'a, TutorialStep>,
        WriteStorage<'a, TutorialUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            actions_storage,
            mut color_storage,
            entity_lookup_storage,
            mut node_storage,
            mut pulse_storage,
            mut rect_storage,
            mut shape_storage,
            mut text_storage,
            tile_nodes_storage,
            mut transform_storage,
            tutorial_step_storage,
            mut tutorial_ui_storage,
        ) = data;

        // TOOD: return out from this function if settings has tutorial completed as true

        let actions = actions_storage.deref();

        let mut details = None;

        let tutorial_step = tutorial_step_storage.deref();

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
                                    if let Some((tile_type, _)) = tile_nodes.nodes.get(&(x2 + x, y2 + y))
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

            details = Some(StepCreationDetails::new(target_cell.0, target_cell.1, size, size, "To start collecting resources, click the glowing tile"));
        } else if actions.action_fired(&TutorialStep::BuildCoal(0.0, 0.0).as_string()) {
            match *tutorial_step {
                TutorialStep::BuildCoal(x, y) => {
                    details = Some(StepCreationDetails::new(
                        x + 10.0,
                        y + 10.0,
                        64.0,
                        64.0,
                        "Click the icon here to build a coal mine operation",
                    ));
                },
                _ => {}
            }
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
                details.message
            );
        }
    }
}
