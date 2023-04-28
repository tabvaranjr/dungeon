use crate::prelude::*;

pub fn hud(
    health_query: Query<&Health, With<Player>>,
    player_query: Query<(Entity, &Player), With<Player>>,
    item_query: Query<(&Name, &Carried), With<Item>>,
) {
    let player_health = health_query.single();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER);

    draw_batch.print_centered(1, "Explore zee Dungeon! Use zee cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        HUD_WIDTH,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );
    draw_batch.print_color_centered(
        0,
        format!("Health: {}/{}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    let (player, map_level) = player_query
        .iter()
        .map(|(entity, player)| (entity, player.map_level))
        .next()
        .unwrap();

    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH * 2, 1),
        format!("Dungeon Level: Over 900{}", map_level + 1),
        ColorPair::new(YELLOW, BLACK),
    );

    let mut y = 3;
    item_query
        .iter()
        .filter(|(_, carried)| carried.0 == player)
        .for_each(|(name, _)| {
            draw_batch.print(Point::new(3, y), format!("{}: {}", y - 2, &name.0));
            y += 1;
        });

    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}
