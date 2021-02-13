mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod state;
mod systems;
mod turn_state;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const HUD_WIDTH: i32 = SCREEN_WIDTH * 2;
    pub const HUD_HEIGHT: i32 = SCREEN_HEIGHT * 2;

    pub const MAP_LAYER: usize = 0;
    pub const DISPLAY_LAYER: usize = 1;
    pub const HUD_LAYER: usize = 2;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::state::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
}

use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawloler")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(HUD_WIDTH, HUD_HEIGHT, "terminal8x8.png")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, state::State::new())
}
