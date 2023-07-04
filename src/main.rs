use std::f32::consts::PI;

use bevy::{prelude::*, window::PresentMode};

// Main function
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "MRE".into(),
                resolution: (500., 300.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(on_update_system)
        .run();
}

// Setup system
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 100.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    let mesh = meshes.add(shape::Quad::new(Vec2::new(25.0, 35.0)).into());
    let material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());

    let card_text = TextBundle::from_section(
        "sample text",
        TextStyle {
            font: asset_server.load("fonts/Belanosima-Regular.ttf"),
            font_size: 100.0,
            color: Color::BLACK,
        },
    );
    let mut card = commands.spawn_empty();

    // 1 true 1 false works
    // both true don't work
    if true { // whether to add text
        card.insert(card_text);
    }
    if true {
        // whether to add Pbr component
        card.insert(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::default().with_rotation(Quat::from_rotation_x(-0.5 * PI)),
            ..default()
        });
    }
}

fn on_update_system(_time: Res<Time>) {}
