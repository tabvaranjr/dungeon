pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
pub const HUD_WIDTH: i32 = SCREEN_WIDTH;
pub const HUD_HEIGHT: i32 = SCREEN_HEIGHT;
pub const HUD_DISPLAY_RATIO: i32 = HUD_WIDTH / DISPLAY_WIDTH;

pub const MAP_LAYER: usize = 0;
pub const DISPLAY_LAYER: usize = 1;
pub const HUD_LAYER: usize = 2;

pub use crate::camera::*;
pub use crate::components::*;
pub use crate::events::*;
pub use crate::map::*;
pub use crate::map_builder::*;
pub use crate::resources::*;
pub use crate::spawner::*;
pub use crate::state::*;
pub use crate::systems::*;
pub use crate::turn_state::*;

pub use bevy_ecs::prelude::{
    Commands, Component, Entity, EventReader, EventWriter, Events, Query, Res, ResMut, Resource,
    Schedule, Stage, StageLabel, SystemStage, With, World, Without
};
pub use bracket_lib::prelude::*;
