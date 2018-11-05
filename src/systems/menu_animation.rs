use rand::{thread_rng, Rng};
use specs::{Join, Read, System, WriteStorage};

use components::{Color, DeltaTime, MenuScreen, Transform};

pub const MENU_ANIMATION_OPTIONS: [((f32, f32), (f32, f32)); 4] = [
    ((0.0, 0.0), (-300.0, -400.0)),
    ((-450.0, -100.0), (0.0, 0.0)),
    ((0.0, -300.0), (-200.0, -100.0)),
    ((-400.0, -350.0), (-100.0, -40.0)),
];

const DURATION: f32 = 2.0;

pub struct MenuAnimation {
    animating_tick: f32,
}

impl MenuAnimation {
    pub fn new() -> Self {
        MenuAnimation {
            animating_tick: 0.0,
        }
    }
}

impl<'a> System<'a> for MenuAnimation {
    type SystemData = (
        WriteStorage<'a, Color>,
        Read<'a, DeltaTime>,
        WriteStorage<'a, MenuScreen>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut color_storage, delta_time_storage, mut menu_screen_storage, mut transform_storage) =
            data;

        let dt = delta_time_storage.dt;

        self.animating_tick += dt;

        if self.animating_tick >= 8.0 {
            self.animating_tick = 0.0;
            for (color, menu_screen, transform) in (
                &mut color_storage,
                &mut menu_screen_storage,
                &mut transform_storage,
            ).join()
            {
                color.0[3] = 1.0;
                menu_screen.animating = false;
                transform.visible = false;
            }
        }

        let mut rng = thread_rng();

        for (color, menu_screen, transform) in (
            &mut color_storage,
            &mut menu_screen_storage,
            &mut transform_storage,
        ).join()
        {
            if !menu_screen.animating && self.animating_tick >= menu_screen.start_time_in_seconds {
                menu_screen.animating = true;
                transform.visible = true;
            } else if menu_screen.animating {
                let diff = (self.animating_tick - menu_screen.start_time_in_seconds) / DURATION;
                if diff < 1.0 {
                    color.0[3] = 1.0 - diff;
                    let start_pos = MENU_ANIMATION_OPTIONS[menu_screen.animation_index].0;
                    let end_pos = MENU_ANIMATION_OPTIONS[menu_screen.animation_index].1;

                    transform.set_pos2(
                        (end_pos.0 - start_pos.0) * diff + start_pos.0,
                        (end_pos.1 - start_pos.1) * diff + start_pos.1,
                    );
                    println!("{:?}", transform.get_pos());
                // animation is done, hide it
                } else if transform.visible {
                    transform.visible = false;
                    // setup for next run
                    menu_screen.animation_index = rng.gen_range(0, MENU_ANIMATION_OPTIONS.len());
                    let pos = MENU_ANIMATION_OPTIONS[menu_screen.animation_index].0;
                    transform.set_pos2(pos.0, pos.1);
                }
            }
        }
    }
}
