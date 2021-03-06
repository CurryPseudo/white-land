use bevy::prelude::*;
use bevy::sprite::entity::SpriteSheetBundle;
use bevy_ase::asset::*;

use bevy_math::Vec2;
use lazy_static::lazy_static;
const TILE_SIZE_X: f32 = 30.0;
const TILE_SIZE_Y: f32 = 16.0;
const UNIT_X: f32 = TILE_SIZE_X + 2.0;
const UNIT_Y: f32 = TILE_SIZE_Y;
lazy_static! {
    static ref X_DIR: Vec2 = Vec2::new(-UNIT_X, -UNIT_Y) / 2.0;
    static ref Y_DIR: Vec2 = Vec2::new(UNIT_X, -UNIT_Y) / 2.0;
}
pub(crate) fn spawn(
    mut commands: Commands,
    animations: Res<Assets<Animation>>,
    //mut sprite_sheet_animations: ResMut<Assets<benimator::SpriteSheetAnimation>>,
) {
    commands.spawn_bundle({
        let mut b = OrthographicCameraBundle::new_2d();
        b.orthographic_projection.scale = 1.0 / 3.0; // scale to 3x
        b
    });

    let (_, iron_land_0) = animations.iter().next().unwrap();
    {
        let anim = iron_land_0;
        let texture_atlas = anim.atlas();
        //let anim: benimator::SpriteSheetAnimation = anim.into();
        //let anim_handle = sprite_sheet_animations.add(anim);
        for i in -30..30 {
            for j in -30..30 {
                commands.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas.clone(),
                    transform: Transform::from_translation(Vec3::from((
                        i as f32 * *X_DIR + j as f32 * *Y_DIR,
                        0.0,
                    ))),
                    ..Default::default()
                });
                //.insert(anim_handle.clone())
                //.insert(benimator::Play);
            }
        }
    }
}
