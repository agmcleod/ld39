use specs::{Component, NullStorage};

#[derive(Default)]
pub struct TutorialUI;

impl Component for TutorialUI {
    type Storage = NullStorage<Self>;
}
