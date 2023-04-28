use crate::prelude::*;

pub fn entity_render(
    renderables: Query<(&Position, &Render)>,
    camera: Res<Camera>,
    fov: Query<&FieldOfView, With<Player>>,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let player_fov = fov.single();
    let offset = Point::new(camera.left_x, camera.top_y);

    renderables
        .iter()
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos.0))
        .for_each(|(pos, render)| {
            draw_batch.set(pos.0 - offset, render.color, render.glyph);
        });

    draw_batch.submit(5000).expect("Batch error");
}
