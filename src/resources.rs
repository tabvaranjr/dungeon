use crate::prelude::*;

#[derive(Resource)]
pub struct KeyCode(pub Option<VirtualKeyCode>);

#[derive(Resource)]
pub struct MousePosition(pub Point);

#[derive(Resource)]
pub struct DynamicMapTheme(pub Box<dyn MapTheme>);
