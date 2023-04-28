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

#[derive(StageLabel)]
enum Stages {
    Update,
    Render,
    Move,
    Combat,
    Events,
    FieldOfView,
}

pub fn build_input_scheduler() -> Schedule {
    let mut scheduler = Schedule::default();

    scheduler.add_stage(Stages::Events, create_event_stage());

    scheduler.add_stage(
        Stages::Update,
        SystemStage::parallel()
            .with_system(player_input::player_input)
            .with_system(fov::fov),
    );

    scheduler.add_stage(
        Stages::Render,
        SystemStage::parallel()
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(tooltips::tooltips)
            .with_system(hud::hud),
    );

    scheduler
}

pub fn build_player_scheduler() -> Schedule {
    let mut scheduler = Schedule::default();

    scheduler.add_stage(Stages::Events, create_event_stage());

    scheduler.add_stage(
        Stages::Combat,
        SystemStage::parallel()
            .with_system(use_items::use_items)
            .with_system(combat::combat),
    );

    scheduler.add_stage(
        Stages::Move,
        SystemStage::parallel().with_system(movement::movement),
    );

    scheduler.add_stage(
        Stages::FieldOfView,
        SystemStage::parallel().with_system(fov::fov),
    );

    scheduler.add_stage(
        Stages::Render,
        SystemStage::parallel()
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(hud::hud)
            .with_system(end_turn::end_turn),
    );

    scheduler
}

pub fn build_monster_scheduler() -> Schedule {
    let mut scheduler = Schedule::default();

    scheduler.add_stage(Stages::Events, create_event_stage());

    scheduler.add_stage(
        Stages::Update,
        SystemStage::parallel()
            .with_system(random_move::random_move)
            .with_system(chasing::chasing),
    );

    scheduler.add_stage(
        Stages::Combat,
        SystemStage::parallel()
            .with_system(use_items::use_items)
            .with_system(combat::combat),
    );

    scheduler.add_stage(
        Stages::Move,
        SystemStage::parallel().with_system(movement::movement),
    );

    scheduler.add_stage(
        Stages::FieldOfView,
        SystemStage::parallel().with_system(fov::fov),
    );

    scheduler.add_stage(
        Stages::Render,
        SystemStage::parallel()
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(hud::hud)
            .with_system(end_turn::end_turn),
    );

    scheduler
}

fn create_event_stage() -> SystemStage {
    SystemStage::parallel()
        .with_system(Events::<WantsToMove>::update_system)
        .with_system(Events::<WantsToAttack>::update_system)
        .with_system(Events::<ActivateItem>::update_system)
}
