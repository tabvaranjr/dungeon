use crate::prelude::*;

pub fn movement(
    mut event_reader: EventReader<WantsToMove>,
    mut map: ResMut<Map>,
    mut camera: ResMut<Camera>,
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    fov: Query<&FieldOfView>,
) {
    for want_move in event_reader.iter() {
        if map.can_enter_tile(want_move.destination) {
            commands
                .entity(want_move.entity)
                .insert(Position(want_move.destination));

            if let Ok(fov) = fov.get(want_move.entity) {
                commands.entity(want_move.entity).insert(fov.clone_dirty());

                if player.get(want_move.entity).is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }
}
