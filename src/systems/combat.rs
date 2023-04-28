use crate::prelude::*;

pub fn combat(
    mut event_reader: EventReader<WantsToAttack>,
    attackers: Query<(Entity, Option<&Damage>), Without<Carried>>,
    mut victims: Query<(Entity, Option<&mut Health>)>,
    weapons: Query<(&Carried, &Damage)>,
    player: Query<Entity, With<Player>>,
    names: Query<(Entity, &Name)>,
    mut commands: Commands,
) {
    for event in event_reader.iter() {
        let attacker = attackers
            .iter()
            .find(|(entity, _)| event.attacker == *entity);

        let is_player_attacker = player.single() == event.attacker;
        let is_player_victim = player.single() == event.victim;

        victims
            .iter_mut()
            .filter(|(entity, _)| event.victim == *entity)
            .for_each(|(entity, health)| {
                let base_damage = if let Some(v) = attacker {
                    if let Some(dmg) = v.1 {
                        dmg.0
                    } else {
                        0
                    }
                } else {
                    0
                };

                let attacker_name = if is_player_attacker {
                    String::from("Player")
                } else {
                    let name = names.iter().find(|(e, _)| *e == event.attacker).map(|(_, n)| n);
                    if let Some(name) = name {
                        name.0.clone()
                    } else {
                        String::from("Unknown")
                    }
                };

                let victim_name = if is_player_victim {
                    String::from("Player")
                } else {
                    let name = names.iter().find(|(e, _)| *e == event.victim).map(|(_, n)| n);
                    if let Some(name) = name {
                        name.0.clone()
                    } else {
                        String::from("Unknown")
                    }
                };

                let weapon_damage: i32 = weapons
                    .iter()
                    .filter(|(carried, _)| carried.0 == attacker.unwrap().0)
                    .map(|(_, dmg)| dmg.0)
                    .sum();

                let final_damage = base_damage + weapon_damage;

                if let Some(mut health) = health {
                    println!(
                        "{} attacks that poor {} for {} damage!",
                        attacker_name, victim_name, final_damage
                    );
                    health.current -= final_damage;
                    println!("{} has {} / {} HP", victim_name, health.current, health.max);

                    if health.current < 1 && !is_player_victim {
                        commands.entity(entity).despawn();
                    }
                }
            });
    }
}
