use crate::prelude::*;

pub fn chasing(
    movers: Query<(Entity, &Position, &FieldOfView), With<ChasingPlayer>>,
    positions: Query<(Entity, &Position), With<Player>>,
    player: Query<&Position, With<Player>>,
    map: Res<Map>,
    mut move_event: EventWriter<WantsToMove>,
    mut attack_event: EventWriter<WantsToAttack>,
) {
    let player_pos = player.single().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map =
        DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, &*map, 1024.0);

    for (entity, pos, fov) in &movers {
        if !fov.visible_tiles.contains(&player_pos) {
            continue;
        }
        let pos = pos.0;
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, &*map) {
            let distance = DistanceAlg::Pythagoras.distance2d(pos, player_pos);
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                player_pos
            };

            let mut attacked = false;
            positions
                .iter()
                .filter(|(_, target_pos)| target_pos.0 == destination)
                .for_each(|(victim, _)| {
                    attack_event.send(WantsToAttack {
                        attacker: entity,
                        victim,
                    });
                    attacked = true;
                });

            if !attacked {
                move_event.send(WantsToMove {
                    entity,
                    destination,
                });
            }
        }
    }
}
