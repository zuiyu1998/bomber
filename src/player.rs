use crate::animate::{SpriteAnimate, SpriteAnimation, SpriteAnimationState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MonsterPlugin;

#[derive(Bundle)]
pub struct MonsterBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    animation: SpriteAnimation,
    animation_state: SpriteAnimationState,
}

impl MonsterBundle {
    pub fn new(
        sprite_sheet_bundle: SpriteSheetBundle,
        animation: SpriteAnimation,
        animation_state: SpriteAnimationState,
    ) -> Self {
        MonsterBundle {
            sprite_sheet_bundle,
            animation,
            animation_state,
        }
    }
}

#[derive(Debug, Component)]
pub struct Player;

fn get_player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut SpriteAnimation, &mut Velocity), With<Player>>,
) {
    let (mut animation, mut velocity) = query.single_mut();

    let mut new_velocity = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        *animation = SpriteAnimation::top();
        new_velocity = Vec2::new(0.0, 16.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        *animation = SpriteAnimation::bottom();
        new_velocity = Vec2::new(0.0, -16.0);
    }
    if keyboard_input.pressed(KeyCode::A) {
        *animation = SpriteAnimation::left();

        new_velocity = Vec2::new(-16.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        *animation = SpriteAnimation::right();
        new_velocity = Vec2::new(16.0, 0.0);
    }

    if new_velocity == Vec2::ZERO {
        animation.0 = animation.0.clone().once();
    } else {
        animation.0 = animation.0.clone().repeat();
    }

    *velocity = Velocity::linear(new_velocity);
}

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(get_player_input.before(SpriteAnimate::SpriteAnimate));
    }
}
