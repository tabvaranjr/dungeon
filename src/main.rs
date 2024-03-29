mod camera;
mod components;
mod events;
mod map;
mod map_builder;
mod prelude;
mod resources;
mod spawner;
mod state;
mod systems;
mod turn_state;

use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawloler")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 16, 16)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(HUD_WIDTH, HUD_HEIGHT, "terminal8x8.png")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
