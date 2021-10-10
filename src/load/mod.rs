use std::path::Path;

use bevy::prelude::*;
use bevy_ase::loader::*;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct AssetLoadStatus(pub(crate) bool);
pub(crate) fn sprites(asset_server: Res<AssetServer>, mut ase_loader: ResMut<Loader>) {
    info!("loading");
    let h = asset_server.load(Path::new("map/iron-land-0.ase"));
    ase_loader.add(h.clone());
}

pub(crate) fn check_sprites(mut state: ResMut<State<AssetLoadStatus>>, ase_loader: Res<Loader>) {
    if ase_loader.is_loaded() {
        info!("load done");
        state.set(AssetLoadStatus(true)).unwrap()
    }
}
