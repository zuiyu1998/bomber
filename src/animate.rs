use benimator::{Animation, FrameRate, State};
use bevy::prelude::*;

#[derive(Debug, Deref, Component)]
pub struct SpriteAnimation(pub Animation);

#[derive(Deref, Component, DerefMut, Default)]
pub struct SpriteAnimationState(State);

pub struct SpriteAnimatePlugin;

#[derive(Debug, SystemLabel)]
pub enum SpriteAnimate {
    SpriteAnimate,
}

impl Plugin for SpriteAnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sprite_animate.label(SpriteAnimate::SpriteAnimate));
    }
}

fn sprite_animate(
    time: Res<Time>,
    mut query: Query<(
        &mut SpriteAnimationState,
        &mut TextureAtlasSprite,
        &SpriteAnimation,
    )>,
) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        player.update(animation, time.delta());

        texture.index = player.sprite_frame_index();
    }
}

impl SpriteAnimation {
    pub fn left() -> Self {
        SpriteAnimation(benimator::Animation::from_indices(
            11..=13,
            FrameRate::from_fps(4.0),
        ))
    }

    pub fn right() -> Self {
        SpriteAnimation(benimator::Animation::from_indices(
            4..=7,
            FrameRate::from_fps(4.0),
        ))
    }

    pub fn top() -> Self {
        SpriteAnimation(benimator::Animation::from_indices(
            [0, 8, 9],
            FrameRate::from_fps(3.0),
        ))
    }

    pub fn bottom() -> Self {
        SpriteAnimation(benimator::Animation::from_indices(
            1..=3,
            FrameRate::from_fps(3.0),
        ))
    }
}
