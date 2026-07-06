//! Listing 25-6：账单一路向上——事件冒泡、propagate(false) 与窗口兜底

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    window: Single<Entity, With<Window>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.8, 6.4).looking_at(Vec3::new(0.0, 0.9, 0.0), Vec3::Y),
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

    // ANCHOR: shelf
    // 货架是父，两件货是子——点货，账单沿父子链一路向上
    let shelf = commands
        .spawn((
            Name::new("货架"),
            Mesh3d(meshes.add(Cuboid::new(4.2, 0.18, 1.6))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.35, 0.26, 0.18),
                perceptual_roughness: 0.8,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.55, 0.0),
        ))
        .observe(ledger)
        .id();
    // ANCHOR_END: shelf

    // ANCHOR: enter_leave
    // 悬停两对事件的冒泡差异，都挂货架上见分晓：
    // Over/Out 逢冒泡必到；Enter/Leave 只看「进没进这片地界」
    commands
        .entity(shelf)
        .observe(|over: On<Pointer<Over>>, names: Query<&Name>| {
            let origin = names.get(over.original_event_target()).map(|n| n.as_str().to_owned());
            println!("货架：Over 到账（起头{}）。", origin.unwrap_or_default());
        })
        .observe(|out: On<Pointer<Out>>, names: Query<&Name>| {
            let origin = names.get(out.original_event_target()).map(|n| n.as_str().to_owned());
            println!("货架：Out 到账（起头{}）。", origin.unwrap_or_default());
        })
        .observe(|_: On<Pointer<Enter>>| {
            println!("货架：Enter——看客头回进本柜地界。");
        })
        .observe(|_: On<Pointer<Leave>>| {
            println!("货架：Leave——看客彻底离柜。");
        });
    // ANCHOR_END: enter_leave

    // ANCHOR: children

    // 鎏金锣：只管记账，账单继续往上送
    commands
        .spawn((
            Name::new("鎏金锣"),
            Mesh3d(meshes.add(Torus::new(0.28, 0.72))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.95, 0.82, 0.55),
                metallic: 1.0,
                perceptual_roughness: 0.25,
                ..default()
            })),
            // 子实体的 Transform 相对父：货架板面在 y=0.55，往上抬 0.75
            Transform::from_xyz(1.2, 0.75, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ChildOf(shelf),
        ))
        .observe(ledger);

    // 琉璃盏：易碎品，账到自己为止——propagate(false) 拦下冒泡
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
            Transform::from_xyz(-1.2, 0.65, 0.0),
            ChildOf(shelf),
        ))
        .observe(|mut click: On<Pointer<Click>>| {
            println!("琉璃盏：易碎——这笔账到我为止，不上货架总账。");
            click.propagate(false);
        });
    // ANCHOR_END: children

    // ANCHOR: window
    // 窗口也是实体：冒泡的终点站，也是「点到空处」的直接收件人
    commands
        .entity(*window)
        .insert(Name::new("台口"))
        .observe(ledger);
    // ANCHOR_END: window
    println!("老雷：账房新规矩——每件货的账，一路抄送到台口。");
}

// ANCHOR: ledger
/// 每一站的账本：当前站（entity）与原始目标（original_event_target）各是谁
fn ledger(click: On<Pointer<Click>>, names: Query<&Name>) {
    let stop = names
        .get(click.entity)
        .map(|n| n.as_str().to_owned())
        .unwrap_or_else(|_| format!("{:?}", click.entity));
    let origin_entity = click.original_event_target();
    let origin = names
        .get(origin_entity)
        .map(|n| n.as_str().to_owned())
        .unwrap_or_else(|_| format!("{origin_entity:?}"));
    println!("场记：{stop}的账本记了一笔——这单起头是{origin}。");
}
// ANCHOR_END: ledger
