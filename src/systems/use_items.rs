use crate::prelude::*;

pub fn use_items(
    mut event_reader: EventReader<ActivateItem>,
    effects: Query<
        (
            Entity,
            Option<&ProvidesHealing>,
            Option<&ProvidesDungeonMap>,
        ),
        With<Item>,
    >,
    mut health_query: Query<(Entity, &mut Health)>,
    mut commands: Commands,
    mut map: ResMut<Map>,
) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    for used_item in event_reader.iter() {
        let item = used_item.item;

        effects
            .iter()
            .filter(|(entity, _, _)| *entity == item)
            .for_each(|(_, healing, mapper)| {
                if let Some(healing) = healing {
                    healing_to_apply.push((used_item.used_by, healing.amount));
                }

                if let Some(_mapper) = mapper {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }
            });

        commands.entity(item).despawn();
    }

    for heal in healing_to_apply.iter() {
        health_query
            .iter_mut()
            .filter(|(entity, _)| *entity == heal.0)
            .for_each(|(_, mut health)| {
                health.current = i32::min(health.max, health.current + heal.1);
            });
    }
}
