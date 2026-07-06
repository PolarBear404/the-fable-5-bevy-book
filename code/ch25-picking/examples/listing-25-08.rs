//! Listing 25-8：拖着挪——DragStart、Drag、DragEnd 与屏幕坐标的方向账

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .init_resource::<RawDelta>()
        .add_systems(Startup, setup)
        .add_systems(Update, switch_feel)
        .run();
}

/// 手感开关：true = 把屏幕 delta 原样加到世界坐标（故意不翻 y）
#[derive(Resource, Default)]
struct RawDelta(bool);

fn setup(
    mut commands: Commands,
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

    let wares: [(&str, Handle<Mesh>, StandardMaterial, Transform); 3] = [
        (
            "琉璃盏",
            meshes.add(Sphere::new(0.55)),
            StandardMaterial {
                base_color: Color::srgba(0.35, 0.62, 0.60, 0.35),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 0.089,
                ..default()
            },
            Transform::from_xyz(-2.0, 1.0, 0.0),
        ),
        (
            "鎏金锣",
            meshes.add(Torus::new(0.28, 0.72)),
            StandardMaterial {
                base_color: Color::srgb(0.95, 0.82, 0.55),
                metallic: 1.0,
                perceptual_roughness: 0.25,
                ..default()
            },
            Transform::from_xyz(0.0, 1.05, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ),
        (
            "剔红漆盒",
            meshes.add(Cuboid::new(0.95, 0.95, 0.95)),
            StandardMaterial {
                base_color: Color::srgb(0.62, 0.11, 0.08),
                perceptual_roughness: 0.35,
                ..default()
            },
            Transform::from_xyz(2.0, 0.98, 0.0),
        ),
    ];
    // ANCHOR: observers
    for (name, mesh, paint, seat) in wares {
        commands
            .spawn((Name::new(name), Mesh3d(mesh), MeshMaterial3d(materials.add(paint)), seat))
            .observe(|start: On<Pointer<DragStart>>, names: Query<&Name>| {
                if let Ok(name) = names.get(start.entity) {
                    println!("场记：{name}离台（{:?} 键拖走）。", start.button);
                }
            })
            .observe(drag_ware)
            .observe(|end: On<Pointer<DragEnd>>, names: Query<&Name>| {
                if let Ok(name) = names.get(end.entity) {
                    println!(
                        "场记：{name}落定——屏幕上共挪了 ({:.0}, {:.0}) 像素。",
                        end.distance.x, end.distance.y
                    );
                }
            });
    }
    // ANCHOR_END: observers
    println!("老雷：陆掌柜要自己摆——按住哪件拖哪件；R 键切「生搬」手感对照。");
}

// ANCHOR: drag
/// 每帧的拖动增量：屏幕像素系（x 向右、y 向下）。
/// 0.008 是屏幕像素到世界单位的粗换算：镜头离台约 6.4 米、竖直视角 45°，
/// 720 像素高的画面装下约 5.3 米世界高度，5.3 / 720 ≈ 0.0074，取整头凑个手感
fn drag_ware(
    drag: On<Pointer<Drag>>,
    mut wares: Query<&mut Transform>,
    raw: Res<RawDelta>,
) {
    let Ok(mut seat) = wares.get_mut(drag.entity) else {
        return;
    };
    seat.translation.x += drag.delta.x * 0.008;
    if raw.0 {
        // 生搬：屏幕 y 朝下，世界 y 朝上——往上拖，货往下走
        seat.translation.y += drag.delta.y * 0.008;
    } else {
        // 翻转 y：这才是「跟手」
        seat.translation.y -= drag.delta.y * 0.008;
    }
}
// ANCHOR_END: drag

fn switch_feel(keys: Res<ButtonInput<KeyCode>>, mut raw: ResMut<RawDelta>) {
    if keys.just_pressed(KeyCode::KeyR) {
        raw.0 = !raw.0;
        println!(
            "小棠：手感切到「{}」。",
            if raw.0 { "生搬 delta——试试往上拖" } else { "翻转 y——跟手" }
        );
    }
}
