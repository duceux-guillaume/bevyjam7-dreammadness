use bevy::prelude::*;

pub(crate) mod fish_level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(fish_level::plugin);
}
