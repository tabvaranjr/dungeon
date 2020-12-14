mod camera;
mod components;
mod map;
mod map_builder;
mod spawners;
mod systems;
mod turn_state;

pub mod prelude {
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
    pub use crate::spawners::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::build(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| {
                spawn_monster(&mut ecs, pos, &mut rng);
            });

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::build(&mut rng);

        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| {
                spawn_monster(&mut self.ecs, pos, &mut rng);
            });

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);

        ctx.print_color_centered(2, RED, BLACK, "VICTOLY!");
        ctx.print_color_centered(4, WHITE, BLACK, "You put on your wizard hat and the amulet of Yala.");
        ctx.print_color_centered(5, WHITE, BLACK, "Power over 9000 course through your veins and you pwn every evil beast.");
        ctx.print_color_centered(6, WHITE, BLACK, "Time for retirement!");
        ctx.print_color_centered(9, GREEN, BLACK, "Press '1' to regret thy life again");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);

        ctx.print_color_centered(2, RED, BLACK, "Thou art dead!");
        ctx.print_color_centered(4, WHITE, BLACK, "Slain and teabagged by a monster, thou art shamed ungracefully.");
        ctx.print_color_centered(5, WHITE, BLACK, "The Amulet of Yala remains unclaimed, and your hometown is getting seriously pwn3d.");
        ctx.print_color_centered(8, YELLOW, BLACK, "Does thy want to get brutally slain again?");
        ctx.print_color_centered(9, GREEN, BLACK, "Press '1' to regret thy life again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(MAP_LAYER);
        ctx.cls();
        ctx.set_active_console(DISPLAY_LAYER);
        ctx.cls();
        ctx.set_active_console(HUD_LAYER);
        ctx.cls();

        // Inputs.
        ctx.set_active_console(MAP_LAYER);
        self.resources.insert(ctx.key);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        // Update.
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => {
                self.input_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => {
                self.monster_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::GameOver => {
                self.game_over(ctx);
            },
            TurnState::Victory => {
                self.victory(ctx);
            }
        };

        // Render.
        render_draw_buffer(ctx).expect("Render error");
    }
}

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
        .build()?;

    main_loop(context, State::new())
}
