use crate::prelude::*;

pub fn random_move(
    movers: Query<(Entity, &Position), With<MovingRandomly>>,
    positions: Query<(Entity, &Position), With<Player>>,
    mut move_event: EventWriter<WantsToMove>,
    mut attack_event: EventWriter<WantsToAttack>,
) {
    for (entity, position) in &movers {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + position.0;

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
