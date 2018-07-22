use cgmath::Vector2;
use components::{Actions, EntityLookup, Node, TileNodes, Pulse, Shape, Tile, TileType, Transform, TutorialStep};
use specs::{Entities, Read, System, WriteStorage};
use std::ops::Deref;
use systems::logic;

pub struct Tutorial;

impl<'a> System<'a> for Tutorial {
    type SystemData = (
        Entities<'a>,
        Read<'a, Actions>,
        Read<'a, EntityLookup>,
        WriteStorage<'a, Node>,
        Read<'a, TileNodes>,
        WriteStorage<'a, Pulse>,
        WriteStorage<'a, Shape>,
        WriteStorage<'a, Transform>,
        Read<'a, TutorialStep>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            actions_storage,
            entity_lookup_storage,
            mut node_storage,
            tile_nodes_storage,
            mut pulse_storage,
            mut shape_storage,
            mut transform_storage,
            tutorial_step_storage,
        ) = data;

        // TOOD: return out from this function if settings has tutorial completed as true

        let actions = actions_storage.deref();
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
                                println!("found {},{}", x, y);
                                target_cell.0 = x as f32;
                                target_cell.1 = y as f32;
                            }
                        }
                    }
                }
            }

            let pulse_shape = entities.create();
            let size = Tile::get_size();
            target_cell.0 *= size;
            target_cell.1 *= size;

            let points = vec![
                Vector2 {
                    x: target_cell.0 - 2.0,
                    y: target_cell.1 - 2.0,
                },
                Vector2 {
                    x: target_cell.0 - 2.0,
                    y: target_cell.1 + size + 2.0,
                },
                Vector2 {
                    x: target_cell.0 + size + 2.0,
                    y: target_cell.1 + size + 2.0,
                },
                Vector2 {
                    x: target_cell.0 + size + 2.0,
                    y: target_cell.1 - 2.0,
                },
            ];

            let shape = Shape::new(points, [1.0, 1.0, 0.0, 0.0], false);
            shape_storage.insert(pulse_shape, shape).unwrap();
            transform_storage
                .insert(pulse_shape, Transform::visible_identity())
                .unwrap();
            pulse_storage.insert(pulse_shape, Pulse::new(2.0)).unwrap();

            let lookup = entity_lookup_storage.deref();
            let node = logic::get_root(&lookup, &mut node_storage);
            node.add(pulse_shape);
        }
    }
}
