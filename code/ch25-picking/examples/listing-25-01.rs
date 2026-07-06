//! Listing 25-1：第一次点中——MeshPickingPlugin 进场，三件货挂上观察者

use bevy::prelude::*;

// ANCHOR: app
fn main() {
    App::new()
        // 拾取管线在 DefaultPlugins 里已经就位，但 mesh 后端不在——要自己请
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}
// ANCHOR_END: app

// ANCHOR: setup
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
    // 台面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.40, 0.42),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 三件货：琉璃盏（半透明的盏身照样点得中——射线只认几何，不认材质）、
    // 鎏金锣（Torus 出厂平躺，立起来对着看客）、剔红漆盒
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
    for (name, mesh, paint, seat) in wares {
        commands
            .spawn((
                Name::new(name),
                Mesh3d(mesh),
                MeshMaterial3d(materials.add(paint)),
                seat,
            ))
            // 拾取事件挂在实体身上：这件货被点中时，这个观察者上场
            .observe(report_click);
    }
    println!("老雷：陆掌柜里边请——今日交货，三件都在台上，能上手。");
    println!("小棠：mesh 拾取已开，指哪件点哪件，场记记账。");
}
// ANCHOR_END: setup

// ANCHOR: observer
/// 点货的回执：目标实体、哪个键、点在世界的哪一点
fn report_click(click: On<Pointer<Click>>, names: Query<&Name>) {
    let name = names
        .get(click.entity)
        .map(|n| n.as_str().to_owned())
        .unwrap_or_else(|_| format!("{:?}", click.entity));
    if let Some(spot) = click.hit.position {
        println!(
            "场记：{name}收到一点——{:?} 键，落点 ({:.2}, {:.2}, {:.2})。",
            click.button, spot.x, spot.y, spot.z
        );
    } else {
        println!("场记：{name}收到一点——{:?} 键，落点没报。", click.button);
    }
}
// ANCHOR_END: observer
