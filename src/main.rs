
use bevy::window::{WindowPlugin, PrimaryWindow, Window};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: bevy::window::WindowResolution::new(1280.0, 720.0),
                title: "Rapier 2d Physics".to_string(),
                present_mode: bevy::window::PresentMode::Fifo,
                mode: bevy::window::WindowMode::Windowed,
                decorations: true,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, maximize_window)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_ball_altitude)
        .add_systems(Update, handle_input)
        .run();
}

// Maximize the window
fn maximize_window(mut query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = query.get_single_mut() {
        window.set_maximized(true);
    } else {
        println!("Failed to get the primary window!");
    }
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))  // collider
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)  // rigid body is a prerequisite in using colliders
        .insert(Collider::ball(50.0))
        .insert(ColliderMassProperties::Density(2.0))
        .insert(Restitution::coefficient(1.42))
        .insert(ExternalForce ::default())
        .insert(ExternalImpulse::default())
        //.insert(GravityScale(0.5))
        .insert(Transform::from_xyz(0.0, 400.0, 0.0));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}


fn handle_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &mut ExternalImpulse, &mut Transform), With<RigidBody>>,
) {

    let delta = time.delta_secs();
    for (mut extrnal_force, mut impulse, mut transform) in query.iter_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
    
        // Apply lift if the user presses the F key
        if keys.pressed(KeyCode::KeyF) {
            if transform.translation.y <= 150.0 {
                impulse.impulse = Vec2::new(0.0, 250.0); // Add upward force
                impulse.torque_impulse = 100.0;
                extrnal_force.force = Vec2::new(0.0, 150.0);
                extrnal_force.torque = 75.0;
                transform.translation.y += 4.5;
                
                println!("Ball altitude on lift: {}", transform.translation.y);
                println!("Lift applied! delta: {:?} impulse.impulse: {:?} impulse.torque_impulse: {:?} extrnal_force.force: {:?}  transform.translation: {:?}", 
                delta, impulse.impulse, impulse.torque_impulse, extrnal_force.force,  transform.translation); // Debug log to confirm key press
            }
        }
    }
   
}