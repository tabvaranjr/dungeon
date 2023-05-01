use crate::prelude::*;

pub struct State {
    pub ecs: World,
    pub input_systems: Schedule,
    pub player_systems: Schedule,
    pub monster_systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let mut ecs = World::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);

        spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);

        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        ecs.insert_resource(map_builder.map);
        ecs.insert_resource(Camera::new(map_builder.player_start));
        ecs.insert_resource(TurnState::AwaitingInput);
        ecs.insert_resource(map_builder.theme);

        ecs.insert_resource(Events::<WantsToMove>::default());
        ecs.insert_resource(Events::<WantsToAttack>::default());
        ecs.insert_resource(Events::<ActivateItem>::default());

        let input_systems = build_input_scheduler();
        let player_systems = build_player_scheduler();
        let monster_systems = build_monster_scheduler();

        Self {
            ecs,
            input_systems,
            player_systems,
            monster_systems,
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.input_systems = build_input_scheduler();
        self.player_systems = build_player_scheduler();
        self.monster_systems = build_monster_scheduler();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut self.ecs, map_builder.player_start);

        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);

        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        self.ecs.insert_resource(map_builder.map);
        self.ecs
            .insert_resource(Camera::new(map_builder.player_start));
        self.ecs.insert_resource(TurnState::AwaitingInput);
        self.ecs.insert_resource(map_builder.theme);

        self.ecs.insert_resource(Events::<WantsToMove>::default());
        self.ecs.insert_resource(Events::<WantsToAttack>::default());
        self.ecs.insert_resource(Events::<ActivateItem>::default());
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);

        ctx.print_color_centered(2, RED, BLACK, "VICTOLY!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on your wizard hat and the amulet of Yala.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Power over 9000 course through your veins and you pwn every evil beast.",
        );
        ctx.print_color_centered(6, WHITE, BLACK, "Time for retirement!");
        ctx.print_color_centered(9, GREEN, BLACK, "Press '1' to regret thy life again");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_LAYER);

        ctx.print_color_centered(2, RED, BLACK, "Thou art dead!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain and teabagged by a monster, thou art shamed ungracefully.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your hometown is getting seriously pwn3d.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Does thy want to get brutally slain again?",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press '1' to regret thy life again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn advance_level(&mut self) {
        let player_entity = self
            .ecs
            .query_filtered::<Entity, With<Player>>()
            .single(&self.ecs);

        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        self.ecs
            .query::<(Entity, &Carried)>()
            .iter(&self.ecs)
            .filter(|(_, carry)| carry.0 == player_entity)
            .map(|(e, _)| e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        let all_entities: Vec<Entity> = self.ecs.query::<Entity>().iter(&self.ecs).collect();
        for e in all_entities {
            if !entities_to_keep.contains(&e) {
                self.ecs.despawn(e);
            }
        }

        self.ecs
            .query::<&mut FieldOfView>()
            .iter_mut(&mut self.ecs)
            .for_each(|mut fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        let mut map_level = 0;
        self.ecs
            .query::<(&mut Player, &mut Position)>()
            .iter_mut(&mut self.ecs)
            .for_each(|(mut player, mut pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.0.x = map_builder.player_start.x;
                pos.0.y = map_builder.player_start.y;
            });

        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );

        self.ecs.insert_resource(map_builder.map);
        self.ecs
            .insert_resource(Camera::new(map_builder.player_start));
        self.ecs.insert_resource(TurnState::AwaitingInput);
        self.ecs.insert_resource(map_builder.theme);
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
        ctx.set_active_console(MAP_LAYER);

        // Execute systems.
        self.ecs.insert_resource(KeyCode(ctx.key));
        self.ecs
            .insert_resource(MousePosition(Point::from_tuple(ctx.mouse_pos())));

        let current_state = self.ecs.get_resource::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => self.input_systems.run(&mut self.ecs),
            TurnState::PlayerTurn => self.player_systems.run(&mut self.ecs),
            TurnState::MonsterTurn => self.monster_systems.run(&mut self.ecs),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
        };

        // Render draw buffers.
        render_draw_buffer(ctx).expect("Render error");
    }
}
