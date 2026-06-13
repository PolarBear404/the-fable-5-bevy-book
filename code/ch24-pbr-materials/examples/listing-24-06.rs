//! Listing 24-6：双面——两面会转的旗，左面是默认的「单面」，背过身就没了；右面双面

use bevy::prelude::*;

#[derive(Component)]
struct Spin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .add_systems(Startup, setup)
        .add_systems(Update, spin)
        .run();
}

// ANCHOR: double_sided
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let flag = meshes.add(Rectangle::new(2.0, 2.6));

    // 左：默认材质。背面被剔除（cull_mode 默认是 Some(Face::Back)），
    // 旗一转到背朝相机，整片就凭空消失
    commands.spawn((
        Mesh3d(flag.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.80, 0.20, 0.18),
            ..default()
        })),
        Transform::from_xyz(-1.7, 1.5, 0.0),
        Spin,
    ));

    // 右：开 double_sided（背面法线自动翻正，两面都正确受光）
    // 再把 cull_mode 设成 None（两面都画，不剔背面）。两样都要，缺一不可
    commands.spawn((
        Mesh3d(flag.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.20, 0.55, 0.80),
            double_sided: true,
            cull_mode: None,
            ..default()
        })),
        Transform::from_xyz(1.7, 1.5, 0.0),
        Spin,
    ));
    // ANCHOR_END: double_sided

    commands.spawn((
        DirectionalLight {
            illuminance: 6000.0,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(0.0, -0.3, -1.0), Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.6, 6.5).looking_at(Vec3::new(0.0, 1.4, 0.0), Vec3::Y),
    ));

    println!("小棠：两面旗一起转——红的背过身就不见了，蓝的开了双面，正反都在。");
}

/// 让两面旗绕竖轴转，好看清正反面的差别
fn spin(mut flags: Query<&mut Transform, With<Spin>>, time: Res<Time>) {
    for mut transform in &mut flags {
        transform.rotate_y(time.delta_secs() * 1.1);
    }
}
