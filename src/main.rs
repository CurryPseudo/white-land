use std::path::Path;

use benimator::*;
use bevy::prelude::*;
use bevy::sprite::entity::SpriteSheetBundle;
use bevy_ase::asset::*;
use bevy_ase::loader::*;
use bevy_math::Vec2;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetLoadStatus(bool);
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AseLoaderDefaultPlugin)
        .add_plugin(AnimationPlugin)
        .add_state(AssetLoadStatus(false))
        .add_system_set(
            SystemSet::on_enter(AssetLoadStatus(false)).with_system(load_sprites.system()),
        )
        .add_system_set(
            SystemSet::on_update(AssetLoadStatus(false))
                .with_system(check_loading_sprites.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AssetLoadStatus(true)).with_system(spawn_sprites.system()),
        )
        .run();
}

pub fn load_sprites(asset_server: Res<AssetServer>, mut ase_loader: ResMut<Loader>) {
    info!("loading");
    let h = asset_server.load(Path::new("map/iron-land-0.ase"));
    ase_loader.add(h.clone());
}

pub fn check_loading_sprites(mut state: ResMut<State<AssetLoadStatus>>, ase_loader: Res<Loader>) {
    if ase_loader.is_loaded() {
        info!("load done");
        state.set(AssetLoadStatus(true)).unwrap()
    }
}

pub fn spawn_sprites(mut commands: Commands, animations: Res<Assets<Animation>>) {
    commands.spawn_bundle({
        let mut b = OrthographicCameraBundle::new_2d();
        b.orthographic_projection.scale = 1.0 / 3.0; // scale to 3x
        b
    });

    let (_, iron_land_0) = animations.iter().next().unwrap();
    let i_dir = Vec2::new(-16.0, -9.0);
    let j_dir = Vec2::new(16.0, -9.0);
    {
        let anim = iron_land_0;
        for i in -3..3 {
            for j in -3..3 {
                commands.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: anim.atlas(),
                    transform: Transform::from_translation(Vec3::from((
                        i as f32 * i_dir + j as f32 * j_dir,
                        0.0,
                    ))),
                    ..Default::default()
                });
            }
        }
    }
}
