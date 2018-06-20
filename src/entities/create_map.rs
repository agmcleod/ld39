use components::TileType;
use rand::{thread_rng, Rng, ThreadRng};
use specs::Entity;
use std::collections::HashMap;

type TileTypeMap = HashMap<(i32, i32), (TileType, Option<Entity>)>;

fn get_random_coord_opposite_of_last(rng: &mut ThreadRng, paths: &Vec<usize>) -> usize {
    if paths.len() > 0 {
        if *(paths.get(paths.len() - 1).unwrap()) == 0 {
            rng.gen_range(0, 5)
        } else {
            rng.gen_range(5, 9)
        }
    } else {
        rng.gen_range(0, 9)
    }
}

fn get_path_index(n: usize) -> usize {
    if n >= 5 {
        1
    } else {
        0
    }
}

fn insert_open_tiles(
    set_nodes: &mut TileTypeMap,
    horizontal: bool,
    x: usize,
    y: usize,
    x_length: usize,
    y_length: usize,
) {
    for x in x..(x + x_length) {
        for y in y..(y + y_length) {
            if horizontal {
                set_nodes.insert((x as i32, y as i32), (TileType::Open, None));
            } else {
                set_nodes.insert((y as i32, x as i32), (TileType::Open, None));
            }
        }
    }
}

fn apply_open_tile_coords(
    set_nodes: &mut TileTypeMap,
    foot_direction_positive: bool,
    which_end_for_short: usize,
    long_length: usize,
    short_length: usize,
    perpendicular_direction_coordinate: usize,
    direction_coordinate: usize,
    horizontal: bool,
) {
    let mut pos = (0, 0);

    pos.1 = if foot_direction_positive {
        perpendicular_direction_coordinate + 2
    } else {
        perpendicular_direction_coordinate - short_length
    };

    pos.0 = if which_end_for_short == 0 {
        direction_coordinate
    } else {
        direction_coordinate + long_length - 2
    };

    insert_open_tiles(
        set_nodes,
        horizontal,
        direction_coordinate,
        perpendicular_direction_coordinate,
        long_length,
        2,
    );

    insert_open_tiles(set_nodes, horizontal, pos.0, pos.1, 2, short_length);
}

pub fn create() -> TileTypeMap {
    let mut rng = thread_rng();

    let mut set_nodes = HashMap::new();
    // used to track what half the last path way was in said direction
    // stores 0 for lower half, 1 for higher half
    let mut horizontal_paths: Vec<usize> = Vec::new();
    let mut vertical_paths: Vec<usize> = Vec::new();
    // build 4 paths
    for _ in 0..4 {
        let horizontal = rng.gen_range(0, 2) == 0;

        let mut long_length = rng.gen_range(2, 11);
        // bias being longer
        if long_length >= 5 {
            long_length = 10;
        } else {
            long_length *= 2;
        }

        let short_length = rng.gen_range(0, 9) / 2;

        // 0 for lower end, 1 for further end
        let which_end_for_short = rng.gen_range(0, 2);

        if horizontal {
            let x = rng.gen_range(0, 11 - long_length);
            let y = get_random_coord_opposite_of_last(&mut rng, &horizontal_paths);

            let positive_facing = y <= 4;

            horizontal_paths.push(get_path_index(y));
            apply_open_tile_coords(
                &mut set_nodes,
                positive_facing,
                which_end_for_short,
                long_length,
                short_length,
                y,
                x,
                horizontal,
            );
        } else {
            let y = rng.gen_range(0, 11 - long_length);
            let x = get_random_coord_opposite_of_last(&mut rng, &vertical_paths);

            let positive_facing = x <= 4;

            vertical_paths.push(get_path_index(x));
            apply_open_tile_coords(
                &mut set_nodes,
                positive_facing,
                which_end_for_short,
                long_length,
                short_length,
                x,
                y,
                horizontal,
            );
        }

        print!("\n");
    }

    set_nodes
}
