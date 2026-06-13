//! Listing 24-5：清漆与镜面——上排拨 clearcoat，下排拨 reflectance，看高光怎么变

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.07)))
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: clearcoat
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball = meshes.add(Sphere::new(0.6));

    // 上排：同一抹深蓝漆，clearcoat 从 0 拨到 1——表面多出一层薄而锐的清漆高光，
    // 像给车壳又罩了层透明亮漆。clearcoat_perceptual_roughness 调这层漆自己的糙度
    for col in 0..4 {
        let coat = col as f32 / 3.0;
        commands.spawn((
            Mesh3d(ball.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.07, 0.10, 0.35),
                perceptual_roughness: 0.5,
                clearcoat: coat,
                clearcoat_perceptual_roughness: 0.1,
                ..default()
            })),
            Transform::from_xyz(col as f32 * 1.5 - 2.25, 1.6, 0.0),
        ));
    }

    // 下排：非金属的镜面强度 reflectance 从弱到强（默认 0.5 ≈ 现实里 4% 的反射），
    // 高光斑随之变亮
    for col in 0..4 {
        let reflect = col as f32 / 3.0;
        commands.spawn((
            Mesh3d(ball.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.55, 0.13, 0.10),
                perceptual_roughness: 0.35,
                reflectance: reflect,
                ..default()
            })),
            Transform::from_xyz(col as f32 * 1.5 - 2.25, 0.2, 0.0),
        ));
    }
    // ANCHOR_END: clearcoat

    // 两盏点光：清漆与镜面的高光都是「镜子」性质的，得有具体光点才照得出来
    commands.spawn((
        PointLight {
            intensity: 3_000_000.0,
            ..default()
        },
        Transform::from_xyz(-3.0, 4.0, 4.0),
    ));
    commands.spawn((
        PointLight {
            intensity: 1_800_000.0,
            ..default()
        },
        Transform::from_xyz(3.5, 2.0, 3.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 6.5).looking_at(Vec3::new(0.0, 0.9, 0.0), Vec3::Y),
    ));

    println!("小棠：上排越往右越像刷了清漆的车壳，多一层亮；下排越往右镜面斑越扎眼。");
}
