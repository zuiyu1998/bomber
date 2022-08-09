use bevy::render::camera::OrthographicProjection;
use bevy::{prelude::*, render::texture::ImageSettings};

pub mod consts;

pub mod debug;
pub mod maps;
pub mod player;
pub mod state;

use consts::MONSTER_Z;
pub use debug::*;
pub use maps::*;
pub use player::*;
pub use state::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(MonsterPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(StatePlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin);

    app.add_startup_system(spawn).run();
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    let projection = OrthographicProjection {
        scale: 0.5,
        ..Default::default()
    };

    commands.spawn_bundle(Camera2dBundle {
        projection,
        ..Camera2dBundle::default()
    });
    let animation = MonsterAnimation::bottom();

    let sprite_sheet_bundle = SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 1,
            custom_size: Some(Vec2::splat(16.0)),
            ..Default::default()
        },
        texture_atlas: textures.add(TextureAtlas::from_grid(
            asset_server.load("creature.png"),
            Vec2::new(16.0, 16.0),
            14,
            1,
        )),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, MONSTER_Z),
            ..Default::default()
        },
        ..Default::default()
    };

    let monster_bundle = MonsterBundle::new(
        sprite_sheet_bundle,
        animation,
        MonsterAnimationState::default(),
    );

    commands.spawn_bundle(monster_bundle).insert(Player);
}
