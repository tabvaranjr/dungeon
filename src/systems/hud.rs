use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    const Z_ORDER: usize = 10000;

    let mut health_query = <&Health>::query().filter(component::<Player>());

    let player_health = health_query.iter(ecs).nth(0).unwrap();

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
        format!("HP: {}/{}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

    draw_batch.submit(Z_ORDER).expect("Batch error");
}
