use crate::prelude::*;
use std::collections::HashSet;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Position(pub Point);

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

#[derive(Component, Clone, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Damage(pub i32);

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Weapon;
