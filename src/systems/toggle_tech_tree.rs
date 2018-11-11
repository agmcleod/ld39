use components::{ui::TutorialUI, upgrade::Buff, Actions, Button, Color, EntityLookup, Input, Node,
                 Rect, ResearchedBuffs, StateChange, Tile, Transform, TutorialStep, Wallet};
use entities::{create_colored_rect, tutorial};
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use state::play_state::PlayState;
use std::ops::{Deref, DerefMut};
use systems::logic;

pub struct ToggleTechTree;

impl ToggleTechTree {
    pub fn new() -> ToggleTechTree {
        ToggleTechTree {}
    }

    fn check_show_tech_tree(
        &mut self,
        lookup: &mut EntityLookup,
        input: &Input,
        entities: &Entities,
        actions_storage: &mut Write<Actions>,
        button_storage: &mut WriteStorage<Button>,
        color_storage: &mut WriteStorage<Color>,
        node_storage: &mut WriteStorage<Node>,
        rect_storage: &mut WriteStorage<Rect>,
        transform_storage: &mut WriteStorage<Transform>,
        tutorial_step_storage: &mut Write<TutorialStep>,
        tutorial_ui_storage: &ReadStorage<TutorialUI>,
        state_change_res: &mut Write<StateChange>,
        tile_storage: &ReadStorage<Tile>,
    ) {
        let mut was_clicked = false;
        {
            let button = button_storage
                .get_mut(*lookup.get("show_button_entity").unwrap())
                .unwrap();
            if button.clicked(&input) {
                {
                    let transform = transform_storage
                        .get_mut(*lookup.get("tech_tree_container").unwrap())
                        .unwrap();
                    transform.visible = true;
                }

                {
                    let transform = transform_storage
                        .get_mut(*lookup.get("side_bar_container").unwrap())
                        .unwrap();
                    transform.visible = false;
                }

                let state_change: &mut StateChange = state_change_res.deref_mut();
                state_change.set(PlayState::get_name(), "tech_tree_pause".to_string());

                let rect = create_colored_rect::create(
                    0.0,
                    0.0,
                    8.0,
                    640,
                    640,
                    [0.0, 0.0, 0.0, 0.8],
                    entities,
                    transform_storage,
                    color_storage,
                    rect_storage,
                );
                lookup.entities.insert("pause_black".to_string(), rect);
                let node = logic::get_root(&lookup, node_storage);
                node.add(rect);

                was_clicked = true;
            }
        }

        if was_clicked {
            tutorial::next_step(
                entities,
                actions_storage,
                tutorial_step_storage,
                tutorial_ui_storage,
                node_storage,
                TutorialStep::ShowUpgrades,
                TutorialStep::Upgrade,
            );
            for (_, button) in (tile_storage, button_storage).join() {
                button.set_disabled(true);
            }
        }
    }

    fn check_resume_from_tech_tree(
        &mut self,
        lookup: &mut EntityLookup,
        input: &Input,
        entities: &Entities,
        actions_storage: &mut Write<Actions>,
        button_storage: &mut WriteStorage<Button>,
        node_storage: &WriteStorage<Node>,
        transform_storage: &mut WriteStorage<Transform>,
        tutorial_step_storage: &mut Write<TutorialStep>,
        tutorial_ui_storage: &ReadStorage<TutorialUI>,
        state_change_res: &mut Write<StateChange>,
        tile_storage: &ReadStorage<Tile>,
    ) {
        let mut was_clicked = false;
        {
            let button = button_storage
                .get_mut(*lookup.get("resume_from_upgrades").unwrap())
                .unwrap();
            if button.clicked(&input) {
                {
                    let transform = transform_storage
                        .get_mut(*lookup.get("tech_tree_container").unwrap())
                        .unwrap();
                    transform.visible = false;
                }

                {
                    let transform = transform_storage
                        .get_mut(*lookup.get("side_bar_container").unwrap())
                        .unwrap();
                    transform.visible = true;
                }

                let state_change: &mut StateChange = state_change_res.deref_mut();
                state_change.set(PlayState::get_name(), "resume".to_string());
                let overlay_entity = *lookup.get("pause_black").unwrap();
                entities.delete(overlay_entity).unwrap();
                lookup.entities.remove("pause_black");
                was_clicked = true;
            }
        }

        if was_clicked {
            tutorial::next_step(
                entities,
                actions_storage,
                tutorial_step_storage,
                tutorial_ui_storage,
                node_storage,
                TutorialStep::Resume,
                TutorialStep::Objective(20.0),
            );
            for (_, button) in (tile_storage, button_storage).join() {
                button.set_disabled(false);
            }
        }
    }
}

impl<'a> System<'a> for ToggleTechTree {
    type SystemData = (
        Entities<'a>,
        Write<'a, Actions>,
        WriteStorage<'a, Button>,
        WriteStorage<'a, Color>,
        Write<'a, EntityLookup>,
        Read<'a, Input>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Rect>,
        Read<'a, ResearchedBuffs>,
        Write<'a, StateChange>,
        ReadStorage<'a, Tile>,
        WriteStorage<'a, Transform>,
        Write<'a, TutorialStep>,
        ReadStorage<'a, TutorialUI>,
        Read<'a, Wallet>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut actions_storage,
            mut button_storage,
            mut color_storage,
            mut lookup,
            input,
            mut node_storage,
            mut rect_storage,
            researched_buffs_storage,
            mut state_change_res,
            tile_storage,
            mut transform_storage,
            mut tutorial_step_storage,
            tutorial_ui_storage,
            wallet_storage,
        ) = data;

        if !researched_buffs_storage
            .0
            .contains_key(&Buff::ResourceTrading)
        {
            // hard cost check. needs to change if i alter cost in tech tree
            if wallet_storage.get_money() >= 50 {
                tutorial::next_step(
                    &entities,
                    &mut actions_storage,
                    &mut tutorial_step_storage,
                    &tutorial_ui_storage,
                    &node_storage,
                    TutorialStep::ResourcesSold,
                    TutorialStep::ShowUpgrades,
                );
            }
        } else if *tutorial_step_storage.deref() == TutorialStep::ResourcesSold {
            tutorial::next_step(
                &entities,
                &mut actions_storage,
                &mut tutorial_step_storage,
                &tutorial_ui_storage,
                &node_storage,
                TutorialStep::ResourcesSold,
                TutorialStep::Objective(20.0),
            );
        }

        let mut lookup: &mut EntityLookup = lookup.deref_mut();
        let input: &Input = input.deref();
        self.check_show_tech_tree(
            &mut lookup,
            &input,
            &entities,
            &mut actions_storage,
            &mut button_storage,
            &mut color_storage,
            &mut node_storage,
            &mut rect_storage,
            &mut transform_storage,
            &mut tutorial_step_storage,
            &tutorial_ui_storage,
            &mut state_change_res,
            &tile_storage,
        );
        self.check_resume_from_tech_tree(
            &mut lookup,
            &input,
            &entities,
            &mut actions_storage,
            &mut button_storage,
            &node_storage,
            &mut transform_storage,
            &mut tutorial_step_storage,
            &tutorial_ui_storage,
            &mut state_change_res,
            &tile_storage,
        );
    }
}
