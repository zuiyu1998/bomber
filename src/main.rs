use benimator::{Animation, FrameRate, State};
use bevy::{prelude::*, render::texture::ImageSettings};

pub struct MonsterResource {
    player: Handle<TextureAtlas>,
}

impl FromWorld for MonsterResource {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let handle = asset_server.load("creature.png");

        MonsterResource { player: handle }
    }
}

pub struct MonsterPlugin;

#[derive(Bundle)]
pub struct MonsterBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    animation: MonsterAnimation,
    animation_state: MonsterAnimationState,
}

impl MonsterBundle {
    pub fn new(
        sprite_sheet_bundle: SpriteSheetBundle,
        animation: MonsterAnimation,
        animation_state: MonsterAnimationState,
    ) -> Self {
        MonsterBundle {
            sprite_sheet_bundle,
            animation,
            animation_state,
        }
    }
}

#[derive(Debug, Deref, Component)]
pub struct MonsterAnimation(Animation);

#[derive(Deref, Component, DerefMut, Default)]
pub struct MonsterAnimationState(State);

#[derive(Debug, Component)]
pub struct Player;

fn monster_animate(
    time: Res<Time>,
    mut query: Query<(
        &mut MonsterAnimationState,
        &mut TextureAtlasSprite,
        &MonsterAnimation,
    )>,
) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        player.update(animation, time.delta());

        texture.index = player.sprite_frame_index();
    }
}

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(monster_animate.label("monster_animate"))
            .init_resource::<MonsterResource>();
    }
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(MonsterPlugin)
        .add_startup_system(spawn)
        .run();
}

fn spawn(mut commands: Commands, monster_resource: Res<MonsterResource>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let sprite_sheet_bundle = SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 0,
            custom_size: Some(Vec2::splat(16.0)),
            ..Default::default()
        },
        texture_atlas: monster_resource.player.clone(),
        ..Default::default()
    };
    let animation = MonsterAnimation(benimator::Animation::from_indices(
        0..=4,
        FrameRate::from_fps(12.0),
    ));

    let monster_bundle = MonsterBundle::new(
        sprite_sheet_bundle,
        animation,
        MonsterAnimationState::default(),
    );

    commands.spawn_bundle(monster_bundle);
}
