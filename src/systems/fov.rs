use crate::prelude::*;

pub fn fov(map: Res<Map>, mut views: Query<(&Position, &mut FieldOfView)>) {
    views
        .iter_mut()
        .filter(|(_, fov)| fov.is_dirty)
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(pos.0, fov.radius, &*map);
            fov.is_dirty = false;
        });
}
