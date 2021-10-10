use benimator::AnimationPlugin;
use bevy::prelude::*;
use bevy_ase::loader::AseLoaderDefaultPlugin;
mod load;
pub(crate) use load::AssetLoadStatus;
mod map;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AseLoaderDefaultPlugin)
        .add_plugin(AnimationPlugin)
        .add_state(AssetLoadStatus(false))
        .add_system_set(
            SystemSet::on_enter(AssetLoadStatus(false)).with_system(load::sprites.system()),
        )
        .add_system_set(
            SystemSet::on_update(AssetLoadStatus(false)).with_system(load::check_sprites.system()),
        )
        .add_system_set(SystemSet::on_enter(AssetLoadStatus(true)).with_system(map::spawn.system()))
        .run();
}
