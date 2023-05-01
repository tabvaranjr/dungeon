use crate::prelude::*;

mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod fov;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;
mod use_items;

use bevy_ecs::schedule::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct FlushEvents;

pub fn build_input_scheduler() -> Schedule {
    let mut scheduler = Schedule::default();

    scheduler.add_systems(
        (
            Events::<WantsToMove>::update_system,
            Events::<WantsToAttack>::update_system,
            Events::<ActivateItem>::update_system,
        )
            .in_set(FlushEvents),
    );

    scheduler.add_systems(
        (
            player_input::player_input,
            fov::fov,
            apply_system_buffers,
            map_render::map_render,
            entity_render::entity_render,
            tooltips::tooltips,
            hud::hud,
        )
            .chain()
            .after(FlushEvents),
    );

    scheduler
}

pub fn build_player_scheduler() -> Schedule {
    let mut scheduler = Schedule::default();

    scheduler.add_systems(
        (
            Events::<WantsToMove>::update_system,
            Events::<WantsToAttack>::update_system,
            Events::<ActivateItem>::update_system,
        )
            .in_set(FlushEvents),
    );

    scheduler.add_systems(
        (
            use_items::use_items,
            combat::combat,
            apply_system_buffers,
            movement::movement,
            apply_system_buffers,
            fov::fov,
            apply_system_buffers,
            map_render::map_render,
            entity_render::entity_render,
            hud::hud,
            end_turn::end_turn,
        )
            .chain()
            .after(FlushEvents),
    );

    scheduler
}

pub fn build_monster_scheduler() -> Schedule {
    let mut scheduler = Schedule::default();

    scheduler.add_systems(
        (
            Events::<WantsToMove>::update_system,
            Events::<WantsToAttack>::update_system,
            Events::<ActivateItem>::update_system,
        )
            .in_set(FlushEvents),
    );

    scheduler.add_systems(
        (
            random_move::random_move,
            chasing::chasing,
            apply_system_buffers,
            use_items::use_items,
            combat::combat,
            apply_system_buffers,
            movement::movement,
            apply_system_buffers,
            fov::fov,
            apply_system_buffers,
            map_render::map_render,
            entity_render::entity_render,
            hud::hud,
            end_turn::end_turn,
        )
            .chain()
            .after(FlushEvents),
    );

    scheduler
}
