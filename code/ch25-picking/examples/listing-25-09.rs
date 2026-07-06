//! Listing 25-9：装箱成交——DragEnter、DragLeave、DragDrop 与「拖起让路」

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}

/// 托盘的记号
#[derive(Component)]
struct Tray;

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

    // ANCHOR: tray
    // 装货的朱漆托盘：DragEnter 亮、DragLeave 还原、DragDrop 成交
    let tray_paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.13, 0.10),
        perceptual_roughness: 0.6,
        ..default()
    });
    let tray_glow = materials.add(StandardMaterial {
        base_color: Color::srgb(0.72, 0.28, 0.16),
        emissive: LinearRgba::rgb(0.25, 0.08, 0.02),
        ..default()
    });
    commands
        .spawn((
            Name::new("托盘"),
            Tray,
            Mesh3d(meshes.add(Cuboid::new(2.2, 0.16, 2.2))),
            MeshMaterial3d(tray_paint.clone()),
            Transform::from_xyz(2.6, 0.58, 0.6),
        ))
        .observe(
            move |enter: On<Pointer<DragEnter>>,
                  names: Query<&Name>,
                  mut coats: Query<&mut MeshMaterial3d<StandardMaterial>>| {
                if let Ok(name) = names.get(enter.dragged) {
                    println!("托盘：{name}悬到我上头了。");
                }
                if let Ok(mut coat) = coats.get_mut(enter.entity) {
                    coat.0 = tray_glow.clone();
                }
            },
        )
        .observe(
            move |leave: On<Pointer<DragLeave>>,
                  names: Query<&Name>,
                  mut coats: Query<&mut MeshMaterial3d<StandardMaterial>>| {
                if let Ok(name) = names.get(leave.dragged) {
                    println!("托盘：{name}又挪开了。");
                }
                if let Ok(mut coat) = coats.get_mut(leave.entity) {
                    coat.0 = tray_paint.clone();
                }
            },
        )
        .observe(
            |drop: On<Pointer<DragDrop>>,
             names: Query<&Name>,
             trays: Query<&Transform, With<Tray>>,
             mut wares: Query<&mut Transform, Without<Tray>>| {
                let Ok(name) = names.get(drop.dropped) else {
                    return;
                };
                println!("托盘：成交——{name}装箱。");
                // 货落托盘正中，摞在板面上
                if let (Ok(tray_seat), Ok(mut ware_seat)) =
                    (trays.get(drop.entity), wares.get_mut(drop.dropped))
                {
                    ware_seat.translation = tray_seat.translation + Vec3::new(0.0, 0.72, 0.0);
                }
            },
        );
    // ANCHOR_END: tray

    let wares: [(&str, Handle<Mesh>, StandardMaterial, Transform); 2] = [
        (
            "琉璃盏",
            meshes.add(Sphere::new(0.55)),
            StandardMaterial {
                base_color: Color::srgba(0.35, 0.62, 0.60, 0.35),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 0.089,
                ..default()
            },
            Transform::from_xyz(-2.4, 1.0, 0.0),
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
            Transform::from_xyz(-0.4, 1.05, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ),
    ];
    // ANCHOR: yield
    for (name, mesh, paint, seat) in wares {
        commands
            .spawn((Name::new(name), Mesh3d(mesh), MeshMaterial3d(materials.add(paint)), seat))
            // 拖起就让路：货自己占着指针正下方，不让开的话
            // 托盘永远轮不到 hover，DragEnter/DragDrop 一辈子不响
            .observe(|start: On<Pointer<DragStart>>, mut commands: Commands| {
                commands.entity(start.entity).insert(Pickable::IGNORE);
            })
            .observe(drag_ware)
            .observe(|end: On<Pointer<DragEnd>>, mut commands: Commands, names: Query<&Name>| {
                commands.entity(end.entity).insert(Pickable::default());
                if let Ok(name) = names.get(end.entity) {
                    println!("场记：{name}落定。");
                }
            });
    }
    // ANCHOR_END: yield
    println!("老雷：验完装箱——拖到朱漆托盘上撒手。");
}

/// 25-8 的跟手版拖动，换算账同款
fn drag_ware(drag: On<Pointer<Drag>>, mut wares: Query<&mut Transform>) {
    if let Ok(mut seat) = wares.get_mut(drag.entity) {
        seat.translation.x += drag.delta.x * 0.008;
        seat.translation.y -= drag.delta.y * 0.008;
    }
}
