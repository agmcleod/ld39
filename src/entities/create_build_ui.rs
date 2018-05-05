use specs::{Entities, WriteStorage};
use scene::Node;
use components::{Button, Color, Rect, ResearchedBuffs, Sprite, TileType, Transform};
use entities::tech_tree::Buff;

pub fn create(
    x: f32,
    y: f32,
    selected_tile_type: &TileType,
    entities: &Entities,
    button_storage: &mut WriteStorage<Button>,
    color_storage: &mut WriteStorage<Color>,
    rect_storage: &mut WriteStorage<Rect>,
    sprite_storage: &mut WriteStorage<Sprite>,
    transform_storage: &mut WriteStorage<Transform>,
    researched_buffs: &ResearchedBuffs,
) -> Node {
    let mut new_entities = Vec::new();

    if *selected_tile_type == TileType::Open {
        let coal_entity = entities.create();
        transform_storage.insert(
            coal_entity,
            Transform::visible(0.0, 0.0, 0.0, 32, 32, 0.0, 1.0, 1.0),
        );
        button_storage.insert(
            coal_entity,
            Button::new(
                "build_coal".to_string(),
                [
                    "wheelbarrel_button_1.png".to_string(),
                    "wheelbarrel_button_2.png".to_string(),
                ],
            ),
        );
        sprite_storage.insert(
            coal_entity,
            Sprite {
                frame_name: "wheelbarrel_button_1.png".to_string(),
            },
        );
        new_entities.push(Node::new(Some(coal_entity), None));
    }

    if researched_buffs.0.contains(&Buff::Oil) && *selected_tile_type == TileType::Open {
        let oil_entity = entities.create();
        transform_storage.insert(
            oil_entity,
            Transform::visible(0.0, 32.0, 0.0, 32, 32, 0.0, 1.0, 1.0),
        );
        button_storage.insert(
            oil_entity,
            Button::new(
                "build_oil".to_string(),
                [
                    "refinery_button_1.png".to_string(),
                    "refinery_button_2.png".to_string(),
                ],
            ),
        );
        sprite_storage.insert(
            oil_entity,
            Sprite {
                frame_name: "refinery_button_1.png".to_string(),
            },
        );

        new_entities.push(Node::new(Some(oil_entity), None));
    }

    if researched_buffs.0.contains(&Buff::Solar) && *selected_tile_type == TileType::Open {
        let solar_entity = entities.create();
        transform_storage.insert(
            solar_entity,
            Transform::visible(0.0, 64.0, 0.0, 32, 32, 0.0, 1.0, 1.0),
        );
        button_storage.insert(
            solar_entity,
            Button::new(
                "build_solar".to_string(),
                [
                    "plant_button_1.png".to_string(),
                    "plant_button_2.png".to_string(),
                ],
            ),
        );
        sprite_storage.insert(
            solar_entity,
            Sprite {
                frame_name: "plant_button_2.png".to_string(),
            },
        );

        new_entities.push(Node::new(Some(solar_entity), None));
    }

    if researched_buffs.0.contains(&Buff::Hydro) && *selected_tile_type == TileType::River {
        let hydro_entity = entities.create();
        transform_storage.insert(
            hydro_entity,
            Transform::visible(0.0, 96.0, 0.0, 32, 32, 0.0, 1.0, 1.0),
        );

        button_storage.insert(
            hydro_entity,
            Button::new(
                "build_hydro".to_string(),
                [
                    "hydro_button.png".to_string(),
                    "hydro_button_2.png".to_string(),
                ]
            )
        );

        sprite_storage.insert(
            hydro_entity,
            Sprite{
                frame_name: "hydro_button.png".to_string(),
            }
        );

        new_entities.push(Node::new(Some(hydro_entity), None));
    }

    let container_entity = entities.create();
    color_storage.insert(container_entity, Color([0.5, 0.5, 0.5, 1.0]));
    rect_storage.insert(container_entity, Rect::new());
    transform_storage.insert(
        container_entity,
        Transform::visible(x, y, 2.0, 32, 32 * new_entities.len() as u16, 0.0, 1.0, 1.0),
    );

    Node::new(Some(container_entity), Some(new_entities))
}
