use std::ops::Deref;
use specs::{Entities, Fetch, Join, System, WriteStorage};
use entities::tech_tree::{TechTreeNode, Upgrade, Status, get_color_from_status, traverse_tree};
use components::Color;
use systems::FRAME_TIME;

pub struct Research;

impl <'a>System<'a> for Research {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Color>,
        Fetch<'a, TechTreeNode>,
        WriteStorage<'a, Upgrade>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut color_storage, tech_tree_storage, mut upgrade_storage) = data;

        let mut upgrade_entities_researched = Vec::with_capacity(3);
        for (entity, color, upgrade) in (&*entities, &mut color_storage, &mut upgrade_storage).join() {
            if upgrade.status == Status::Learning {
                upgrade.current_research_progress += FRAME_TIME;
                if upgrade.current_research_progress >= upgrade.time_to_research {
                    println!("Done research");
                    upgrade.status = Status::Researched;
                    upgrade_entities_researched.push(entity);
                    *color = Color(get_color_from_status(&upgrade.status));
                }
            }
        }

        for upgrade_entity_researched in &upgrade_entities_researched {
            let tech_tree: &TechTreeNode = tech_tree_storage.deref();
            let mut unlock_next_nodes = |node: &TechTreeNode| {
                if node.entity == *upgrade_entity_researched {
                    for sub_node in &node.sub_nodes {
                        upgrade_storage.get_mut(sub_node.entity).unwrap().status = Status::Researchable;
                    }
                    return true
                }

                false
            };
            traverse_tree(&tech_tree, &mut unlock_next_nodes);
        }
    }
}
