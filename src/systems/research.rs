use components::{upgrade::{Buff, LearnProgress, Status, Upgrade, UpgradeLinesLookup},
                 Color,
                 DeltaTime,
                 ResearchedBuffs,
                 ResearchingEntities,
                 Shape,
                 Transform};
use entities::tech_tree::{get_color_from_status, traverse_tree, TechTreeNode};
use scene::Node;
use specs::{Entities, Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

type ResearchingUpgrades = HashMap<Buff, (f32, f32)>;

pub struct Research {
    scene: Arc<Mutex<Node>>,
}

impl Research {
    pub fn new(scene: Arc<Mutex<Node>>) -> Self {
        Research { scene }
    }

    fn research_finished(&self, buff: Buff, researched_buffs: &mut ResearchedBuffs) {
        researched_buffs.0.insert(buff.clone());
    }

    fn update_progress_ui<'a>(
        &mut self,
        entities: Entities,
        mut transform_storage: WriteStorage<'a, Transform>,
        learn_progress_storage: &ReadStorage<'a, LearnProgress>,
        researching_upgrades: &ResearchingUpgrades,
        researching_entities: &mut ResearchingEntities,
    ) {
        let mut progress_entities_removed = false;

        for (entity, transform, learn_progress) in
            (&*entities, &mut transform_storage, learn_progress_storage).join()
        {
            if let Some(progress_time) = researching_upgrades.get(&learn_progress.buff) {
                transform.size.x = (32.0 * (progress_time.0 / progress_time.1)) as u16;
                if progress_time.0 / progress_time.1 >= 1.0 {
                    let mut scene = self.scene.lock().unwrap();
                    entities.delete(entity).unwrap();
                    scene.remove_node_with_entity(&entities, entity);
                    let entity_position = researching_entities
                        .entities
                        .iter()
                        .position(|e| *e == entity)
                        .unwrap();
                    researching_entities.entities.remove(entity_position);
                    progress_entities_removed = true;
                }
            }
        }

        if progress_entities_removed {
            // loop through updated array set to put in positions
            for (i, entity) in researching_entities.entities.iter().enumerate() {
                let mut transform = transform_storage.get_mut(*entity).unwrap();
                let y = transform.get_pos().y;
                transform.set_pos2(20.0 + 40.0 * i as f32, y);
            }
        }
    }
}

impl<'a> System<'a> for Research {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Color>,
        Read<'a, DeltaTime>,
        ReadStorage<'a, LearnProgress>,
        Write<'a, ResearchedBuffs>,
        Write<'a, ResearchingEntities>,
        WriteStorage<'a, Shape>,
        ReadExpect<'a, TechTreeNode>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Upgrade>,
        Read<'a, UpgradeLinesLookup>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut color_storage,
            delta_time_storage,
            learn_progress_storage,
            mut researched_buffs,
            mut researching_entities_storage,
            mut shape_storage,
            tech_tree_storage,
            transform_storage,
            mut upgrade_storage,
            upgrade_lines_lookup,
        ) = data;

        let dt = delta_time_storage.deref().dt;

        let mut upgrade_entities_researched = Vec::with_capacity(3);
        let mut researching_entities = researching_entities_storage.deref_mut();
        let mut researching_upgrades: ResearchingUpgrades = HashMap::new();
        for (entity, color, upgrade) in
            (&*entities, &mut color_storage, &mut upgrade_storage).join()
        {
            if upgrade.status == Status::Learning {
                upgrade.current_research_progress += dt;
                if upgrade.current_research_progress >= upgrade.time_to_research {
                    self.research_finished(upgrade.buff, researched_buffs.deref_mut());
                    upgrade.status = Status::Researched;
                    upgrade_entities_researched.push(entity);
                    *color = Color(get_color_from_status(&upgrade.status));
                }
                researching_upgrades.insert(
                    upgrade.buff,
                    (upgrade.current_research_progress, upgrade.time_to_research),
                );
            }
        }

        self.update_progress_ui(
            entities,
            transform_storage,
            &learn_progress_storage,
            &researching_upgrades,
            &mut researching_entities,
        );

        for upgrade_entity_researched in &upgrade_entities_researched {
            let tech_tree: &TechTreeNode = tech_tree_storage.deref();
            let mut unlock_next_nodes = |node: &TechTreeNode| {
                if node.entity == *upgrade_entity_researched {
                    // update colour of sub node line bars
                    if let Some(line_entities) = upgrade_lines_lookup.entities.get(&node.entity) {
                        for entity in line_entities {
                            let mut shape = shape_storage.get_mut(*entity).unwrap();
                            shape.buffers =
                                Shape::build_buffers(shape.points.clone(), [0.7, 0.7, 0.7, 1.0]);
                        }
                    }
                    for sub_node in &node.sub_nodes {
                        upgrade_storage.get_mut(sub_node.entity).unwrap().status =
                            Status::Researchable;
                        color_storage
                            .insert(
                                sub_node.entity,
                                Color(get_color_from_status(&Status::Researchable)),
                            )
                            .unwrap();
                    }
                    return true;
                }

                false
            };
            traverse_tree(&tech_tree, &mut unlock_next_nodes);
        }
    }
}
