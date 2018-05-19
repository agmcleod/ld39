use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use specs::{Entities, Join, ReadExpect, ReadStorage, System, Write, WriteStorage};
use entities::tech_tree::{get_color_from_status, traverse_tree, TechTreeNode};
use components::{Color, ResearchedBuffs, ResearchingCount, Transform,
                 upgrade::{Buff, LearnProgress, Status, Upgrade}};
use systems::FRAME_TIME;
use scene::Node;

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
}

impl<'a> System<'a> for Research {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Color>,
        ReadStorage<'a, LearnProgress>,
        Write<'a, ResearchedBuffs>,
        Write<'a, ResearchingCount>,
        ReadExpect<'a, TechTreeNode>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Upgrade>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut color_storage,
            learn_progress_storage,
            mut researched_buffs,
            mut researching_count_storage,
            tech_tree_storage,
            mut transform_storage,
            mut upgrade_storage,
        ) = data;

        let mut upgrade_entities_researched = Vec::with_capacity(3);
        let researching_count = researching_count_storage.deref_mut();
        let mut researching_entities = HashMap::new();
        for (entity, color, upgrade) in
            (&*entities, &mut color_storage, &mut upgrade_storage).join()
        {
            if upgrade.status == Status::Learning {
                upgrade.current_research_progress += FRAME_TIME;
                if upgrade.current_research_progress >= upgrade.time_to_research {
                    self.research_finished(upgrade.buff, researched_buffs.deref_mut());
                    upgrade.status = Status::Researched;
                    upgrade_entities_researched.push(entity);
                    *color = Color(get_color_from_status(&upgrade.status));
                    researching_count.count -= 1;
                }
                researching_entities.insert(
                    upgrade.buff,
                    (upgrade.current_research_progress, upgrade.time_to_research),
                );
            }
        }

        for (entity, transform, learn_progress) in
            (&*entities, &mut transform_storage, &learn_progress_storage).join()
        {
            if let Some(progress_time) = researching_entities.get(&learn_progress.buff) {
                transform.size.x = (32.0 * (progress_time.0 / progress_time.1)) as u16;
                if progress_time.0 / progress_time.1 >= 1.0 {
                    let mut scene = self.scene.lock().unwrap();
                    entities.delete(entity).unwrap();
                    scene.remove_node_with_entity(&entities, entity);
                }
            }
        }

        for upgrade_entity_researched in &upgrade_entities_researched {
            let tech_tree: &TechTreeNode = tech_tree_storage.deref();
            let mut unlock_next_nodes = |node: &TechTreeNode| {
                if node.entity == *upgrade_entity_researched {
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
