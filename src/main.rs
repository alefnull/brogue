mod entities;
mod state;
mod util;

use std::time::{SystemTime, UNIX_EPOCH};

use bracket_lib::prelude::*;
use entities::entity::EntityFuncs;
use state::State;

fn main() -> BError {
    let ratio = Point::new(16, 9);
    let width = 5 * ratio.x;
    let height = 5 * ratio.y;

    let context = BTermBuilder::simple(width, height)
        .expect("Failed to create context")
        .with_fps_cap(60.0)
        .with_tile_dimensions(16, 16)
        .with_title("BROGUE")
        .build()?;

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get time since epoch for seed")
        .as_secs();

    let mut gs = State::new(width as u32, height as u32, seed);
    gs.map.generate(1);

    gs.player
        .set_pos(gs.map.width as i32 / 2, gs.map.height as i32 / 2);

    main_loop(context, gs)
}
