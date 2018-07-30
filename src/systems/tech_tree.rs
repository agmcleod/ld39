use components::ui;
use components::{ui::{TutorialUI, WalletUI},
                 upgrade::{Buff, LearnProgress},
                 Actions,
                 Color,
                 EntityLookup,
                 Input,
                 Node,
                 Rect,
                 ResearchingEntities,
                 Sprite,
                 Text,
                 Transform,
                 TutorialStep,
                 Wallet};
use entities::tech_tree::{get_color_from_status, Status, Upgrade};
use entities::{create_text, create_tooltip, tutorial};
use specs::{Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};
use storage_types::*;
use systems::logic;

pub struct TechTree {
    current_tooltip: Option<Entity>,
    current_tech_tree_node_entity: Option<Entity>,
}

impl TechTree {
    pub fn new() -> TechTree {
        TechTree {
            current_tooltip: None,
            current_tech_tree_node_entity: None,
        }
    }

    fn build_research_progress_ui(
        &self,
        buff: Buff,
        lookup: &EntityLookup,
        entities: &Entities,
        sprite: Sprite,
        color_storage: &mut WriteStorage<Color>,
        learn_progress_storage: &mut WriteStorage<LearnProgress>,
        node_storage: &mut WriteStorage<Node>,
        rect_storage: &mut WriteStorage<Rect>,
        sprite_storage: &mut WriteStorage<Sprite>,
        transform_storage: &mut WriteStorage<Transform>,
        researching_count: usize,
    ) -> Entity {
        let sprite_entity = entities.create();
        sprite_storage.insert(sprite_entity, sprite).unwrap();
        transform_storage
            .insert(
                sprite_entity,
                Transform::visible(0.0, -36.0, 0.0, 32, 32, 0.0, 1.0, 1.0),
            )
            .unwrap();

        let progress_entity = entities.create();
        transform_storage
            .insert(
                progress_entity,
                Transform::visible(
                    20.0 + 40.0 * researching_count as f32,
                    546.0,
                    0.0,
                    0,
                    10,
                    0.0,
                    1.0,
                    1.0,
                ),
            )
            .unwrap();
        color_storage
            .insert(progress_entity, Color([0.0, 1.0, 0.0, 1.0]))
            .unwrap();
        rect_storage.insert(progress_entity, Rect {}).unwrap();
        learn_progress_storage
            .insert(progress_entity, LearnProgress { buff })
            .unwrap();

        let mut node = Node::new();
        node.add(sprite_entity);
        node_storage.insert(progress_entity, node).unwrap();

        let sidebar_entity = lookup.get("side_bar_container").unwrap();
        let sidebar_node = node_storage.get_mut(*sidebar_entity).unwrap();
        sidebar_node.add(progress_entity);
        progress_entity
    }
}

impl<'a> System<'a> for TechTree {
    type SystemData = (
        Entities<'a>,
        Write<'a, Actions>,
        WriteStorage<'a, Color>,
        Read<'a, EntityLookup>,
        Read<'a, Input>,
        WriteStorage<'a, LearnProgress>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Rect>,
        Write<'a, ResearchingEntities>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, ui::TechTreeButton>,
        WriteStorage<'a, Text>,
        Write<'a, TutorialStep>,
        ReadStorage<'a, TutorialUI>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Upgrade>,
        Write<'a, Wallet>,
        ReadStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut actions_storage,
            mut color_storage,
            entity_lookup_storage,
            input_storage,
            mut learn_progress_storage,
            mut node_storage,
            mut rect_storage,
            mut researching_entities_storage,
            mut sprite_storage,
            tech_tree_node_storage,
            mut text_storage,
            mut tutorial_step_storage,
            tutorial_ui_storage,
            mut transform_storage,
            mut upgrade_storage,
            mut wallet_storage,
            wallet_ui_storage,
        ) = data;

        let input: &Input = input_storage.deref();
        let lookup: &EntityLookup = entity_lookup_storage.deref();

        let mut mouse_over_tech_tree_node_entity = None;
        let mut tooltip_position = [0.0, 0.0];

        let root_entity = lookup.get("root").unwrap();

        for (entity, _, transform) in
            (&*entities, &tech_tree_node_storage, &transform_storage).join()
        {
            let absolute_pos =
                Node::get_absolute_pos(root_entity, &entity, &transform_storage, &node_storage);
            let abs_transform = Transform::visible(
                absolute_pos.x,
                absolute_pos.y,
                0.0,
                transform.size.x,
                transform.size.y,
                transform.rotation,
                transform.scale.x,
                transform.scale.y,
            );
            if abs_transform.contains(&input.mouse_pos.0, &input.mouse_pos.1) {
                mouse_over_tech_tree_node_entity = Some(entity.clone());
                tooltip_position[0] = transform.get_pos().x;
                tooltip_position[1] = transform.get_pos().y;
            }
        }

        if let Some(mouse_over_tech_tree_node_entity) = mouse_over_tech_tree_node_entity {
            let create_tooltip =
                if let Some(current_tech_tree_node_entity) = self.current_tech_tree_node_entity {
                    current_tech_tree_node_entity != mouse_over_tech_tree_node_entity
                } else {
                    true
                };

            if create_tooltip {
                if let Some(current_tooltip) = self.current_tooltip {
                    entities.delete(current_tooltip).unwrap();
                    self.current_tooltip = None;
                    self.current_tech_tree_node_entity = None;
                }
                let container_entity = *lookup.get(&"tech_tree_container".to_string()).unwrap();

                let tech_tree_node_ui = tech_tree_node_storage
                    .get(mouse_over_tech_tree_node_entity)
                    .unwrap();
                let upgrade = upgrade_storage
                    .get(mouse_over_tech_tree_node_entity)
                    .unwrap();

                let (container_w, container_h) = {
                    let transform = transform_storage.get(container_entity).unwrap();
                    (transform.size.x, transform.size.y)
                };

                let mut tooltip_entity = create_tooltip::create(
                    &entities,
                    &mut color_storage,
                    &mut node_storage,
                    &mut rect_storage,
                    &mut text_storage,
                    &mut transform_storage,
                    tooltip_position[0] - 70.0,
                    tooltip_position[1] + 32.0,
                    container_w as i32,
                    container_h as i32,
                    160,
                    160,
                    tech_tree_node_ui.text.clone(),
                    None,
                );
                self.current_tooltip = Some(tooltip_entity.clone());
                self.current_tech_tree_node_entity = Some(mouse_over_tech_tree_node_entity.clone());

                let mut text_storage_type = TextStorage {
                    entities: &entities,
                    color_storage: &mut color_storage,
                    text_storage: &mut text_storage,
                    transform_storage: &mut transform_storage,
                };

                {
                    let tooltip_node = node_storage.get_mut(tooltip_entity).unwrap();
                    if upgrade.status == Status::Researched {
                        let text = create_text::create(
                            &mut text_storage_type,
                            "Researched".to_string(),
                            20.0,
                            5.0,
                            130.0,
                            0.0,
                            120,
                            20,
                            Color([0.5, 0.5, 0.5, 1.0]),
                        );
                        tooltip_node.add(text);
                    } else {
                        let text = create_text::create(
                            &mut text_storage_type,
                            format!("${}", tech_tree_node_ui.cost),
                            20.0,
                            5.0,
                            130.0,
                            0.0,
                            70,
                            20,
                            Color([1.0, 1.0, 0.0, 1.0]),
                        );
                        tooltip_node.add(text);

                        let time_left = if upgrade.status == Status::Learning {
                            upgrade.time_to_research - upgrade.current_research_progress
                        } else {
                            upgrade.time_to_research
                        };

                        let text = create_text::create(
                            &mut text_storage_type,
                            format!("{} sec", time_left),
                            20.0,
                            100.0,
                            130.0,
                            0.0,
                            70,
                            20,
                            Color([1.0, 1.0, 0.0, 1.0]),
                        );
                        tooltip_node.add(text);
                    }
                }

                if let Some(container_node) = node_storage.get_mut(container_entity) {
                    container_node.add(tooltip_entity);
                }
            } else if input.mouse_pressed {
                let wallet: &mut Wallet = wallet_storage.deref_mut();
                let upgrade = upgrade_storage
                    .get_mut(mouse_over_tech_tree_node_entity)
                    .unwrap();
                if upgrade.status == Status::Researchable && wallet.spend(upgrade.cost) {
                    if upgrade.buff == Buff::ResourceTrading {
                        tutorial::next_step(
                            &entities,
                            &mut actions_storage,
                            &mut tutorial_step_storage,
                            &tutorial_ui_storage,
                            TutorialStep::Upgrade,
                            TutorialStep::Resume,
                        );
                    }
                    upgrade.start_learning();
                    *color_storage
                        .get_mut(mouse_over_tech_tree_node_entity)
                        .unwrap() = Color(get_color_from_status(&upgrade.status));
                    let researching_entities = researching_entities_storage.deref_mut();
                    let sprite = (*sprite_storage
                        .get(mouse_over_tech_tree_node_entity)
                        .unwrap())
                        .clone();
                    let progress_entity = self.build_research_progress_ui(
                        upgrade.buff,
                        &lookup,
                        &entities,
                        sprite,
                        &mut color_storage,
                        &mut learn_progress_storage,
                        &mut node_storage,
                        &mut rect_storage,
                        &mut sprite_storage,
                        &mut transform_storage,
                        researching_entities.entities.len(),
                    );
                    researching_entities.entities.push(progress_entity);
                    logic::update_text(
                        format!("{}", wallet.money),
                        &mut text_storage,
                        &wallet_ui_storage,
                    );
                }
            }
        } else {
            if let Some(current_tooltip) = self.current_tooltip {
                entities.delete(current_tooltip).unwrap();
                self.current_tooltip = None;
                self.current_tech_tree_node_entity = None;
            }
        }
    }
}
