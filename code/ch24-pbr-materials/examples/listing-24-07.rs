//! Listing 24-7：深度偏移——两片贴在同一平面上，左边争着出现打架闪烁，右边给标签 +depth_bias

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.07, 0.08, 0.10)))
        .add_systems(Startup, setup)
        .run();
}

// ANCHOR: depth_bias
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let board = meshes.add(Rectangle::new(2.6, 2.6));
    let label = meshes.add(Rectangle::new(1.5, 1.5));
    let board_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.33, 0.40),
        unlit: true,
        ..default()
    });

    for (x, bias) in [(-1.7_f32, 0.0_f32), (1.7, 1.0)] {
        // 底板
        commands.spawn((
            Mesh3d(board.clone()),
            MeshMaterial3d(board_mat.clone()),
            Transform::from_xyz(x, 1.3, 0.0),
        ));
        // 标签：和底板「同一个 z」，正贴在一起。两片深度相等、谁压谁没了准 —— 相机一动，
        // 时而这片赢、时而那片赢，便是 z-fighting 的闪烁；这里 bias = 0 的左边，标签干脆被
        // 底板吞了。给标签一点正的 depth_bias（右边），它就稳稳压在底板前面
        commands.spawn((
            Mesh3d(label.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.95, 0.65, 0.20),
                unlit: true,
                depth_bias: bias,
                ..default()
            })),
            Transform::from_xyz(x, 1.3, 0.0),
        ));
    }
    // ANCHOR_END: depth_bias

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.3, 6.5).looking_at(Vec3::new(0.0, 1.3, 0.0), Vec3::Y),
    ));

    println!("小棠：左边俩贴一块儿，橙标签让底板吞了——相机一动还直打架；右边加了 depth_bias，老实压在上头。");
}
