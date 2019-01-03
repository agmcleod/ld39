use components::{Button, Color, EffectedByPollutionTiles, GathererType, Node, Rect,
                 ResearchedBuffs, Sprite, Text, TileType, Transform};
use entities::{create_colored_rect, create_text, tech_tree::Buff};
use renderer;
use specs::{Entities, Entity, LazyUpdate, Read, WriteStorage};
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
    lazy: &'a Read<LazyUpdate>,
    node_storage: &mut WriteStorage<'b, Node>,
    researched_buffs: &ResearchedBuffs,
) -> (Entity, f32, f32) {
    let mut new_entities = Vec::new();

    if *selected_tile_type == TileType::Open {
        let coal_entity = lazy.create_entity(entities)
            .with(Transform::visible(SPACING_F, SPACING_F, 1.0, SIZE, SIZE, 0.0, 1.0, 1.0))
            .with(
                Button::new(
                    "build_coal".to_string(),
                    [
                        "mine_button_1.png".to_string(),
                        "mine_button_2.png".to_string(),
                    ],
                )
            )
            .with(
                Sprite {
                    frame_name: "mine_button_1.png".to_string(),
                }
            )
            .with(EffectedByPollutionTiles::new())
            .build();

        new_entities.push(coal_entity);
        let text = create_text::create(
            entities,
            lazy,
            format!("${}", GathererType::Coal.get_build_cost()),
            16.0,
            29.0,
            79.0,
            0.0,
            50,
            20,
            Color([0.0, 1.0, 0.0, 1.0]),
            None,
        );
        new_entities.push(text);
    }

    if researched_buffs.0.contains_key(&Buff::Oil) && *selected_tile_type == TileType::Open {
        let oil_entity = lazy.create_entity(entities)
            .with(
                Transform::visible(
                    SPACING_F,
                    CELL_HEIGHT as f32,
                    0.0,
                    SIZE,
                    SIZE,
                    0.0,
                    1.0,
                    1.0,
                )
            )
            .with(
                Button::new(
                    "build_oil".to_string(),
                    [
                        "refinery_button_1.png".to_string(),
                        "refinery_button_2.png".to_string(),
                    ],
                )
            )
            .with(
                Button::new(
                    "build_oil".to_string(),
                    [
                        "refinery_button_1.png".to_string(),
                        "refinery_button_2.png".to_string(),
                    ],
                )
            )
            .with(
                Sprite {
                    frame_name: "refinery_button_1.png".to_string(),
                }
            )
            .with(EffectedByPollutionTiles::new())
            .build();

        new_entities.push(oil_entity);
        let text = create_text::create(
            entities,
            lazy,
            format!("${}", GathererType::Oil.get_build_cost()),
            16.0,
            29.0,
            173.0,
            0.0,
            50,
            20,
            Color([0.0, 1.0, 0.0, 1.0]),
            None,
        );
        new_entities.push(text);
    }

    if researched_buffs.0.contains_key(&Buff::Solar) && *selected_tile_type == TileType::Open {
        let solar_entity = lazy.create_entity(&entities)
            .with(Transform::visible(CELL_WIDTH as f32, SPACING_F, 0.0, SIZE, SIZE, 0.0, 1.0, 1.0))
            .with(
                Button::new(
                    "build_solar".to_string(),
                    [
                        "plant_button_1.png".to_string(),
                        "plant_button_2.png".to_string(),
                    ],
                )
            )
            .with(
                Sprite {
                    frame_name: "plant_button_2.png".to_string(),
                }
            )
            .with(EffectedByPollutionTiles::new())
            .build();

        new_entities.push(solar_entity);
        let mut cost = GathererType::Solar.get_build_cost();
        if researched_buffs
            .0
            .contains_key(&Buff::PurchaseSolarCellCompany)
        {
            cost -= cost * 20 / 100;
        }
        let text = create_text::create(
            entities,
            lazy,
            format!("${}", cost),
            16.0,
            102.0,
            79.0,
            0.0,
            50,
            20,
            Color([0.0, 1.0, 0.0, 1.0]),
            None,
        );
        new_entities.push(text);
    }

    if researched_buffs.0.contains_key(&Buff::Hydro) && *selected_tile_type == TileType::River {
        let hydro_entity = lazy.create_entity(&entities)
            .with(
                Transform::visible(SPACING_F, SPACING_F, 0.0, SIZE, SIZE, 0.0, 1.0, 1.0)
            )
            .with(
                Button::new(
                    "build_hydro".to_string(),
                    [
                        "hydro_button.png".to_string(),
                        "hydro_button_2.png".to_string(),
                    ],
                )
            )
            .with(
                Sprite {
                    frame_name: "hydro_button.png".to_string(),
                }
            )
            .with(EffectedByPollutionTiles::new())
            .build();

        new_entities.push(hydro_entity);
        let text = create_text::create(
            entities,
            lazy,
            format!("${}", GathererType::Hydro.get_build_cost()),
            16.0,
            29.0,
            79.0,
            0.0,
            50,
            20,
            Color([0.0, 1.0, 0.0, 1.0]),
            None,
        );
        new_entities.push(text);
    }

    let dim = renderer::get_dimensions();

    let x = cmp::max(
        0,
        cmp::min(x as i32, dim[0] as i32 - CONTAINER_WIDTH as i32),
    ) as f32;

    let y = cmp::max(
        0,
        cmp::min(y as i32, dim[1] as i32 - CONTAINER_HEIGHT as i32),
    ) as f32;

    let container_entity = create_colored_rect::create(
        x,
        y,
        4.0,
        CONTAINER_WIDTH,
        CONTAINER_HEIGHT,
        [0.0, 0.0, 0.0, 0.8],
        entities,
        lazy,
    );

    let mut node = Node::new();
    node.add_many(new_entities);
    node_storage.insert(container_entity, node).unwrap();

    (container_entity, x, y)
}
