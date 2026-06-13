//! Listing 25-3：用 `Pickable` 和 `MeshPickingSettings::require_markers` 把拾取范围收窄。

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        // ANCHOR: require_markers
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ..default()
        })
        // ANCHOR_END: require_markers
        .insert_resource(ClearColor(Color::srgb(0.05, 0.06, 0.08)))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct PickLog(&'static str);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let picked = materials.add(Color::srgb(0.34, 0.75, 0.95));
    let ignored = materials.add(Color::srgb(0.35, 0.35, 0.38));

    // ANCHOR: camera_and_targets
    commands.spawn((
        Camera3d::default(),
        MeshPickingCamera,
        Transform::from_xyz(0.0, 3.8, 7.5).looking_at(Vec3::new(0.0, 0.7, 0.0), Vec3::Y),
    ));

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.2, 1.2, 1.2))),
            MeshMaterial3d(picked.clone()),
            Transform::from_xyz(-1.2, 0.8, 0.0),
            Pickable::default(),
            PickLog("左边这只箱笼会响应点击"),
        ))
        .observe(report_click);

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 1.2, 1.2))),
        MeshMaterial3d(ignored.clone()),
        Transform::from_xyz(1.2, 0.8, 0.0),
        // 有 Pickable 组件，但显式不 hover、不挡住下面的东西。
        Pickable::IGNORE,
    ));
    // ANCHOR_END: camera_and_targets

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(7.0, 7.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.15, 0.16, 0.17))),
        Pickable::IGNORE,
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 1_800_000.0,
            ..default()
        },
        Transform::from_xyz(3.0, 6.0, 4.0),
    ));

    // ANCHOR: non_blocking_panel
    let font = asset_server.load("fonts/book-sans-sc-regular.otf");
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: px(82),
            left: px(0),
            top: px(0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.28)),
        Pickable {
            should_block_lower: false,
            ..default()
        },
        children![(
            Text::new("这条 UI 面板自己可被拾取，但不挡住下面的 mesh"),
            TextFont {
                font,
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Pickable::IGNORE,
        )],
    ));
    // ANCHOR_END: non_blocking_panel
}

fn report_click(event: On<Pointer<Click>>, query: Query<&PickLog>) {
    if let Ok(log) = query.get(event.entity) {
        println!("{}", log.0);
    }
}
