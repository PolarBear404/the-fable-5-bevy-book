//! Listing 24-4：透明五调——同一抹半透明橙，alpha_mode 不同，叠在背景墙上各是各的算法

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.06, 0.07, 0.09)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 背景：一面冷灰墙（unlit，颜色均匀），给前排透明片当统一的「背后」
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(12.0, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.45, 0.50, 0.60),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.4, -0.8),
    ));

    let pane = meshes.add(Rectangle::new(1.4, 2.2));
    let orange = Color::srgba(0.95, 0.55, 0.15, 0.5);

    // ANCHOR: alpha
    // 镂空那片用一张带 alpha 的贴图；其余四片用同一抹半透明橙，只换 alpha_mode
    let lattice = asset_server.load("textures/lattice.png");
    let panes: [(AlphaMode, Color, Option<Handle<Image>>); 5] = [
        // 默认 Opaque：alpha 通道被无视，照样不透明（最常见的「我设了透明怎么没用」）
        (AlphaMode::Opaque, orange, None),
        // Mask：按阈值一刀切，非透即不透——做镂空、栅栏、树叶最省（不用排序）
        (AlphaMode::Mask(0.5), Color::WHITE, Some(lattice)),
        // Blend：标准 alpha 混合，玻璃、水
        (AlphaMode::Blend, orange, None),
        // Add：与背后相加，越加越亮——发光、全息、激光
        (AlphaMode::Add, orange, None),
        // Multiply：与背后相乘，越乘越暗——染色玻璃、滤色片
        (AlphaMode::Multiply, orange, None),
    ];
    for (i, (mode, color, texture)) in panes.into_iter().enumerate() {
        commands.spawn((
            Mesh3d(pane.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                base_color_texture: texture,
                alpha_mode: mode,
                unlit: true,
                ..default()
            })),
            Transform::from_xyz(i as f32 * 1.6 - 3.2, 1.4, 0.4),
        ));
    }
    // ANCHOR_END: alpha

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.4, 6.5).looking_at(Vec3::new(0.0, 1.4, 0.0), Vec3::Y),
    ));

    println!("小棠：五片同色的橙——头一片把透明吃了，后面几片各透各的：镂空、玻璃、发光、滤色。");
}
