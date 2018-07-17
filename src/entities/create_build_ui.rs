use components::{Button, Color, GathererType, Node, Rect, ResearchedBuffs, Sprite, Text, TileType, Transform};
use entities::{create_colored_rect, tech_tree::Buff, create_text};
use renderer;
use specs::{Entities, Entity, WriteStorage};
use storage_types::TextStorage;
use std::cmp;

const SPACING_F: f32 = 10.0;
const SPACING: u16 = SPACING_F as u16;
const SIZE: u16 = 64;
const CELL_WIDTH: u16 = SIZE + SPACING * 2;
const CELL_HEIGHT: u16 = SIZE + SPACING + 30;

const CONTAINER_WIDTH: u16 = 160;
const CONTAINER_HEIGHT: u16 = 200;

pub fn create<'a, 'b: 'a>(
    x: f32,
    y: f32,
    selected_tile_type: &TileType,
    entities: &'a Entities,
    button_storage: &mut WriteStorage<'b, Button>,
    color_storage: &mut WriteStorage<'b, Color>,
    node_storage: &mut WriteStorage<'b, Node>,
    rect_storage: &mut WriteStorage<'b, Rect>,
    sprite_storage: &mut WriteStorage<'b, Sprite>,
    text_storage: &mut WriteStorage<'b, Text>,
    transform_storage: &mut WriteStorage<'b, Transform>,
    researched_buffs: &ResearchedBuffs,
) -> Entity {
    let mut new_entities = Vec::new();

    if *selected_tile_type == TileType::Open {
        let coal_entity = entities.create();
        transform_storage
            .insert(
                coal_entity,
                Transform::visible(SPACING_F, SPACING_F, 1.0, SIZE, SIZE, 0.0, 1.0, 1.0),
            )
            .unwrap();
        button_storage
            .insert(
                coal_entity,
                Button::new(
                    "build_coal".to_string(),
                    [
                        "wheelbarrel_button_1.png".to_string(),
                        "wheelbarrel_button_2.png".to_string(),
                    ],
                ),
            )
            .unwrap();
        sprite_storage
            .insert(
                coal_entity,
                Sprite {
                    frame_name: "wheelbarrel_button_1.png".to_string(),
                },
            )
            .unwrap();
        new_entities.push(coal_entity);
        let text = create_text::create(
            &mut TextStorage{ entities, color_storage, text_storage, transform_storage },
            format!("${}", GathererType::Coal.get_build_cost()),
            16.0,
            29.0,
            79.0,
            0.0,
            50,
            20,
            Color([0.0, 1.0, 0.0, 1.0])
        );
        new_entities.push(text);
    }

    if researched_buffs.0.contains(&Buff::Oil) && *selected_tile_type == TileType::Open {
        let oil_entity = entities.create();
        transform_storage
            .insert(
                oil_entity,
                Transform::visible(SPACING_F, CELL_HEIGHT as f32, 0.0, SIZE, SIZE, 0.0, 1.0, 1.0),
            )
            .unwrap();
        button_storage
            .insert(
                oil_entity,
                Button::new(
                    "build_oil".to_string(),
                    [
                        "refinery_button_1.png".to_string(),
                        "refinery_button_2.png".to_string(),
                    ],
                ),
            )
            .unwrap();
        sprite_storage
            .insert(
                oil_entity,
                Sprite {
                    frame_name: "refinery_button_1.png".to_string(),
                },
            )
            .unwrap();

        new_entities.push(oil_entity);
        let text = create_text::create(
            &mut TextStorage{ entities, color_storage, text_storage, transform_storage },
            format!("${}", GathererType::Oil.get_build_cost()),
            16.0,
            29.0,
            173.0,
            0.0,
            50,
            20,
            Color([0.0, 1.0, 0.0, 1.0])
        );
        new_entities.push(text);
    }

    if researched_buffs.0.contains(&Buff::Solar) && *selected_tile_type == TileType::Open {
        let solar_entity = entities.create();
        transform_storage
            .insert(
                solar_entity,
                Transform::visible(CELL_WIDTH as f32, SPACING_F, 0.0, SIZE, SIZE, 0.0, 1.0, 1.0),
            )
            .unwrap();
        button_storage
            .insert(
                solar_entity,
                Button::new(
                    "build_solar".to_string(),
                    [
                        "plant_button_1.png".to_string(),
                        "plant_button_2.png".to_string(),
                    ],
                ),
            )
            .unwrap();
        sprite_storage
            .insert(
                solar_entity,
                Sprite {
                    frame_name: "plant_button_2.png".to_string(),
                },
            )
            .unwrap();

        new_entities.push(solar_entity);
        let text = create_text::create(
            &mut TextStorage{ entities, color_storage, text_storage, transform_storage },
            format!("${}", GathererType::Oil.get_build_cost()),
            16.0,
            102.0,
            79.0,
            0.0,
            50,
            20,
            Color([0.0, 1.0, 0.0, 1.0])
        );
        new_entities.push(text);
    }

    if researched_buffs.0.contains(&Buff::Hydro) && *selected_tile_type == TileType::River {
        let hydro_entity = entities.create();
        transform_storage
            .insert(
                hydro_entity,
                Transform::visible(SPACING_F, SPACING_F, 0.0, SIZE, SIZE, 0.0, 1.0, 1.0),
            )
            .unwrap();

        button_storage
            .insert(
                hydro_entity,
                Button::new(
                    "build_hydro".to_string(),
                    [
                        "hydro_button.png".to_string(),
                        "hydro_button_2.png".to_string(),
                    ],
                ),
            )
            .unwrap();

        sprite_storage
            .insert(
                hydro_entity,
                Sprite {
                    frame_name: "hydro_button.png".to_string(),
                },
            )
            .unwrap();

        new_entities.push(hydro_entity);
    }

    let dim = renderer::get_dimensions();

    let x = cmp::max(
        0, cmp::min(x as i32, dim[0] as i32 - CONTAINER_WIDTH as i32)
    );

    let y = cmp::max(
        0, cmp::min(y as i32, dim[1] as i32 - CONTAINER_HEIGHT as i32)
    );

    let container_entity = create_colored_rect::create(
        x as f32,
        y as f32,
        4.0,
        CONTAINER_WIDTH,
        CONTAINER_HEIGHT,
        [0.0, 0.0, 0.0, 0.8],
        entities,
        transform_storage,
        color_storage,
        rect_storage
    );

    let mut node = Node::new();
    node.add_many(new_entities);
    node_storage.insert(container_entity, node).unwrap();

    container_entity
}
