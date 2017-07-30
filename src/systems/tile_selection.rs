use std::ops::Deref;
use specs::{ReadStorage, WriteStorage, Fetch, Join, System};
use components::{Gatherer, HighlightTile, Input, SelectedTile, Tile, Transform};

pub struct TileSelection;

impl<'a> System<'a> for TileSelection {
    type SystemData = (
        ReadStorage<'a, Gatherer>,
        WriteStorage<'a, HighlightTile>,
        Fetch<'a, Input>,
        WriteStorage<'a, SelectedTile>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gatherer_storage, mut hightlight_tile_storage, input_storage, mut selected_tile_storage, mut transform_storage) = data;

        let input: &Input = input_storage.deref();
        let mouse_x = input.mouse_pos.0;
        let mouse_y = 640 - input.mouse_pos.1;
        let within_grid = mouse_x >= 0 && mouse_x <= 640 && mouse_y >= 0 && mouse_y <= 640;
        let tile_size = Tile::get_size();
        let tile_mouse_x = mouse_x / tile_size * tile_size;
        let tile_mouse_y = mouse_y / tile_size * tile_size;

        for (hightlight_tile, transform) in (&mut hightlight_tile_storage, &mut transform_storage).join() {
            if within_grid {
                transform.pos.x = tile_mouse_x;
                transform.pos.y = tile_mouse_y;
                hightlight_tile.visible = true;
            } else {
                hightlight_tile.visible = false;
            }
        }

        if input.mouse_pressed && within_grid {
            let mut collisions = false;

            for (_, transform) in (&gatherer_storage, &mut transform_storage).join() {
                if transform.pos.x == tile_mouse_x && transform.pos.y == tile_mouse_y {
                    collisions = true;
                    break
                }
            }

            if !collisions {
                for (selected_tile, transform) in (&mut selected_tile_storage, &mut transform_storage).join() {
                    selected_tile.visible = true;
                    transform.pos.x = tile_mouse_x;
                    transform.pos.y = tile_mouse_y;
                }
            }
        }
    }
}

