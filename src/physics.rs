use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct TilePhysicBundle {
    #[bundle]
    pub transform_bundle: TransformBundle,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

impl TilePhysicBundle {
    pub fn new(local_transform: Transform, tile_size: &TilemapTileSize) -> Self {
        let transform_bundle = TransformBundle {
            local: local_transform,
            ..Default::default()
        };

        let rigid_body = RigidBody::Fixed;

        let collider = Collider::cuboid(tile_size.x / 2.0, tile_size.y / 2.0);

        TilePhysicBundle {
            transform_bundle,
            rigid_body,
            collider,
        }
    }
}

pub struct PhysicPlugin;

impl Plugin for PhysicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_system(display_events);
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
