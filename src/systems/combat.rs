use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        let is_player_victim = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let attacker_name = if let Ok(attacker) = ecs.entry_ref(*attacker) {
            if attacker.get_component::<Player>().is_ok() {
                String::from("Player")
            } else if let Ok(name) = attacker.get_component::<Name>() {
                name.0.clone()
            } else {
                String::from("Unknown")
            }
        } else {
            String::from("Unknown")
        };

        let victim_name = if let Ok(victim) = ecs.entry_ref(*victim) {
            if victim.get_component::<Player>().is_ok() {
                String::from("Player")
            } else if let Ok(name) = victim.get_component::<Name>() {
                name.0.clone()
            } else {
                String::from("Unknown")
            }
        } else {
            String::from("Unknown")
        };

        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();

        let final_damage = base_damage + weapon_damage;

        println!(
            "{} attacks {} for {} damage.",
            attacker_name, victim_name, final_damage
        );

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;
            if health.current < 1 && !is_player_victim {
                commands.remove(*victim);
            }
        }

        commands.remove(*message);
    });
}
