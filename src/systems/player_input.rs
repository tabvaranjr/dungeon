use crate::prelude::*;

pub fn player_input(
    player: Query<(Entity, &Position, &mut Health), With<Player>>,
    enemies: Query<(Entity, &Position), With<Enemy>>,
    items: Query<(Entity, &Position, Option<&Weapon>), With<Item>>,
    carried_items: Query<(Entity, &Carried), With<Item>>,
    carried_weapons: Query<(Entity, &Carried), With<Weapon>>,
    key: Res<KeyCode>,
    mut turn_state: ResMut<TurnState>,
    mut move_event: EventWriter<WantsToMove>,
    mut attack_event: EventWriter<WantsToAttack>,
    mut use_event: EventWriter<ActivateItem>,
    mut commands: Commands,
) {
    if let Some(key) = key.0 {
        let delta = match key {
            VirtualKeyCode::Escape => std::process::exit(0),
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                let (player, player_pos, _) = player.single();
                items
                    .iter()
                    .filter(|(_, &item_pos, _)| item_pos == *player_pos)
                    .for_each(|(item, _, weapon)| {
                        commands.entity(item).remove::<Position>();
                        commands.entity(item).insert(Carried(player));

                        if weapon.is_some() {
                            carried_weapons
                                .iter()
                                .filter(|(_, c)| c.0 == player)
                                .for_each(|(e, _)| {
                                    commands.entity(e).despawn();
                                });
                        }
                    });

                Point::zero()
            }
            VirtualKeyCode::Key1 => use_item(0, &player, &carried_items, &mut use_event),
            VirtualKeyCode::Key2 => use_item(1, &player, &carried_items, &mut use_event),
            VirtualKeyCode::Key3 => use_item(2, &player, &carried_items, &mut use_event),
            VirtualKeyCode::Key4 => use_item(3, &player, &carried_items, &mut use_event),
            VirtualKeyCode::Key5 => use_item(4, &player, &carried_items, &mut use_event),
            VirtualKeyCode::Key6 => use_item(5, &player, &carried_items, &mut use_event),
            VirtualKeyCode::Key7 => use_item(6, &player, &carried_items, &mut use_event),
            VirtualKeyCode::Key8 => use_item(7, &player, &carried_items, &mut use_event),
            VirtualKeyCode::Key9 => use_item(8, &player, &carried_items, &mut use_event),
            _ => Point::zero(),
        };

        let (player_entity, destination) = player
            .iter()
            .map(|(entity, pos, _)| (entity, pos.0 + delta))
            .next()
            .unwrap();

        let mut did_something = false;
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter()
                .filter(|(_, pos)| pos.0 == destination)
                .for_each(|(enemy_entity, _)| {
                    hit_something = true;
                    did_something = true;

                    attack_event.send(WantsToAttack {
                        attacker: player_entity,
                        victim: enemy_entity,
                    })
                });

            if !hit_something {
                move_event.send(WantsToMove {
                    entity: player_entity,
                    destination,
                });
            }
        }

        *turn_state = TurnState::PlayerTurn;
    }
}

fn use_item(
    n: usize,
    player: &Query<(Entity, &Position, &mut Health), With<Player>>,
    items: &Query<(Entity, &Carried), With<Item>>,
    use_event: &mut EventWriter<ActivateItem>,
) -> Point {
    let (player_entity, _, _) = player.single();

    let item_entity = items
        .iter()
        .filter(|(_, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, _)| *item_count == n)
        .map(|(_, (item_entity, _))| item_entity)
        .next();

    if let Some(item_entity) = item_entity {
        use_event.send(ActivateItem {
            used_by: player_entity,
            item: item_entity,
        });
    }

    Point::zero()
}
