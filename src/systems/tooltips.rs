use crate::prelude::*;

pub fn tooltips(
    positions: Query<(&Position, &Name, Option<&Health>)>,
    mouse_pos: Res<MousePosition>,
    camera: Res<Camera>,
    fov: Query<&FieldOfView, With<Player>>,
) {
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = mouse_pos.0 + offset;
    let player_fov = fov.single();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER);

    positions
        .iter()
        .filter(|(pos, _, _)| pos.0 == map_pos && player_fov.visible_tiles.contains(&pos.0))
        .for_each(|(_, name, health)| {
            let screen_pos = mouse_pos.0 * HUD_DISPLAY_RATIO;
            let display = if let Some(health) = health {
                format!("{}: {} HP", name.0, health.current)
            } else {
                name.0.clone()
            };

            draw_batch.print(screen_pos, &display);
        });

    draw_batch.submit(10100).expect("Batch error");
}
