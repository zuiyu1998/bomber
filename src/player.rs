use benimator::{Animation, FrameRate, State};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
pub struct MonsterAnimation(pub Animation);

#[derive(Deref, Component, DerefMut, Default)]
pub struct MonsterAnimationState(State);

impl MonsterAnimation {
    pub fn left() -> Self {
        MonsterAnimation(benimator::Animation::from_indices(
            11..=13,
            FrameRate::from_fps(4.0),
        ))
    }

    pub fn right() -> Self {
        MonsterAnimation(benimator::Animation::from_indices(
            4..=7,
            FrameRate::from_fps(4.0),
        ))
    }

    pub fn top() -> Self {
        MonsterAnimation(benimator::Animation::from_indices(
            [0, 8, 9],
            FrameRate::from_fps(3.0),
        ))
    }

    pub fn bottom() -> Self {
        MonsterAnimation(benimator::Animation::from_indices(
            1..=3,
            FrameRate::from_fps(3.0),
        ))
    }
}

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

fn get_player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut MonsterAnimation, &mut Velocity), With<Player>>,
) {
    let (mut animation, mut velocity) = query.single_mut();

    let mut new_velocity = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        *animation = MonsterAnimation::top();
        new_velocity = Vec2::new(0.0, 16.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        *animation = MonsterAnimation::bottom();
        new_velocity = Vec2::new(0.0, -16.0);
    }
    if keyboard_input.pressed(KeyCode::A) {
        *animation = MonsterAnimation::left();

        new_velocity = Vec2::new(-16.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        *animation = MonsterAnimation::right();
        new_velocity = Vec2::new(16.0, 0.0);
    }
    *velocity = Velocity::linear(new_velocity);
}

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(monster_animate.label("monster_animate"))
            .add_system(get_player_input.before("monster_animate"));
    }
}
