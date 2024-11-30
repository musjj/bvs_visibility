use bevy::{
    math::vec3,
    prelude::*,
    render::{
        primitives::Aabb,
        view::{check_visibility, VisibilitySystems},
    },
};
use bevy_vector_shapes::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, Shape2dPlugin::default()))
        .add_systems(Startup, setup_shape)
        .add_systems(Update, move_shape)
        .add_systems(
            PostUpdate,
            (
                check_visibility::<With<ShapeMaterial>>.in_set(VisibilitySystems::CheckVisibility),
                check_shape_visibility.after(VisibilitySystems::CheckVisibility),
            ),
        )
        .run()
}

#[derive(Component)]
struct MyShape;

fn setup_shape(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        MyShape,
        Aabb::from_min_max(vec3(-20.0, -20.0, 0.0), vec3(20.0, 20.0, 0.0)),
        ShapeBundle::circle(&ShapeConfig::default_2d(), 20.0),
    ));
}

fn move_shape(time: Res<Time>, mut transform: Single<&mut Transform, With<MyShape>>) {
    transform.translation.x += time.delta_secs() * 250.0;
}

fn check_shape_visibility(vis: Single<&ViewVisibility, With<MyShape>>) {
    if vis.get() {
        info!("shape is visible")
    } else {
        info!("shape is not visible")
    }
}
