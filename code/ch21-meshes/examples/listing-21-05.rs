//! Listing 21-5：上漆——金属感与粗糙度，两根旋钮拧出一面材质墙

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: grid
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball = meshes.add(Sphere::new(0.5));

    // 同一款金漆，十五种调法：
    // 横轴粗糙度 0.1 → 1.0，纵轴金属感 0.0 → 1.0
    for (row, metallic) in [0.0_f32, 0.5, 1.0].into_iter().enumerate() {
        for col in 0..5 {
            let roughness = 0.1 + col as f32 * 0.225;
            commands.spawn((
                Mesh3d(ball.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.86, 0.62, 0.32),
                    metallic,
                    perceptual_roughness: roughness,
                    ..default()
                })),
                Transform::from_xyz(col as f32 * 1.3 - 2.6, row as f32 * 1.3 - 1.3, 0.0),
            ));
        }
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 3.0, 4.0)));

    println!("小棠：一缸金漆调十五个样——横着越来越糙，竖着越来越金。");
}
// ANCHOR_END: grid
