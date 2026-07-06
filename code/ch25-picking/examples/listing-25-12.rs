//! Listing 25-12：第三个后端——UI 牌子天生可点，还压着 3D 的地界

use bevy::prelude::*;

fn main() {
    App::new()
        // UI 后端也随 DefaultPlugins 自动就位
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, drill_hole)
        .run();
}

#[derive(Component)]
struct Sign;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.8, 6.4).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.0, 6.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.40, 0.42),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));
    // 琉璃盏坐台，等着被牌子挡
    commands
        .spawn((
            Name::new("琉璃盏"),
            Mesh3d(meshes.add(Sphere::new(0.55))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.35, 0.62, 0.60, 0.35),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 0.089,
                ..default()
            })),
            Transform::from_xyz(-2.0, 1.0, 0.0),
        ))
        .observe(|click: On<Pointer<Click>>, names: Query<&Name>| {
            if let Ok(name) = names.get(click.entity) {
                println!("场记：{name}收到一点。");
            }
        });

    // ANCHOR: sign
    // 一块 UI 木牌，钉在琉璃盏的正前方（屏幕坐标定位——Node 的细节 28 章再算）。
    // UI 节点天生参与拾取，事件挂法与 3D 一模一样
    commands
        .spawn((
            Sign,
            Text::new("上手验货"),
            TextFont {
                font: asset_server.load("fonts/book-sans-sc-regular.otf").into(),
                font_size: FontSize::Px(34.0),
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                left: px(300),
                top: px(295),
                ..default()
            },
        ))
        .observe(|over: On<Pointer<Over>>, mut colors: Query<&mut TextColor>| {
            if let Ok(mut color) = colors.get_mut(over.entity) {
                color.0 = Color::srgb(0.98, 0.86, 0.35);
            }
        })
        .observe(|out: On<Pointer<Out>>, mut colors: Query<&mut TextColor>| {
            if let Ok(mut color) = colors.get_mut(out.entity) {
                color.0 = Color::WHITE;
            }
        })
        .observe(|_: On<Pointer<Click>>| {
            println!("木牌：收到一点。");
        });
    // ANCHOR_END: sign
    println!("老雷：柜上钉了块字牌，正好挡着琉璃盏——先点点牌子，再按 U 给它开洞。");
}

// ANCHOR: drill
/// U 键给牌子开洞：不挡下家，自己照收——一点两账，UI 与 mesh 两个后端各记一笔
fn drill_hole(keys: Res<ButtonInput<KeyCode>>, mut commands: Commands, sign: Single<Entity, With<Sign>>) {
    if keys.just_pressed(KeyCode::KeyU) {
        commands
            .entity(*sign)
            .insert(Pickable { should_block_lower: false, is_hoverable: true });
        println!("小棠：牌子开洞——不挡下家，自己照收。");
    }
}
// ANCHOR_END: drill
