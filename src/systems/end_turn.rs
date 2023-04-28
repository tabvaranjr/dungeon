use crate::prelude::*;

pub fn end_turn(
    mut turn_state: ResMut<TurnState>,
    player: Query<(&Position, &Health), With<Player>>,
    amulet: Query<&Position, With<AmuletOfYala>>,
    map: Res<Map>,
) {
    let current_state = *turn_state;
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    let (pos, hp) = player.single();
    if hp.current < 1 {
        new_state = TurnState::GameOver;
    }

    let amulet_default = Position(Point::new(-1, -1));
    let amulet_pos = amulet.get_single().unwrap_or(&amulet_default).0;
    if pos.0 == amulet_pos {
        new_state = TurnState::Victory;
    }

    let idx = map.point2d_to_index(pos.0);
    if map.tiles[idx] == TileType::Exit {
        new_state = TurnState::NextLevel;
    }

    *turn_state = new_state;
}
