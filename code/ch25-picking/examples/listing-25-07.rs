//! Listing 25-7：纱幕的四种规矩——Pickable 的两个开关拨全四档

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, switch_mode)
        .run();
}

/// 纱幕的记号，切档系统靠它找人
#[derive(Component)]
struct Veil;

// ANCHOR: modes
/// 两个布尔开关，二二得四档
const MODES: [(&str, Pickable); 4] = [
    (
        "守门（默认）——挡下家，自己收",
        Pickable { should_block_lower: true, is_hoverable: true },
    ),
    ("隐身（IGNORE）——不挡下家，自己不收", Pickable::IGNORE),
    (
        "吸音——挡下家，自己不收",
        Pickable { should_block_lower: true, is_hoverable: false },
    ),
    (
        "通透——不挡下家，自己也收",
        Pickable { should_block_lower: false, is_hoverable: true },
    ),
];
// ANCHOR_END: modes

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

    // 幕后的货：鎏金锣一件足矣
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
            Transform::from_xyz(0.0, 1.05, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ))
        .observe(report_click);

    // ANCHOR: veil
    // 纱幕立在镜头与货之间。半透明只是它的长相——
    // 射线不问材质，纱幕挡不挡拾取，全看 Pickable 怎么填
    commands
        .spawn((
            Name::new("纱幕"),
            Veil,
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::new(2.4, 1.2)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.55, 0.65, 0.70, 0.30),
                alpha_mode: AlphaMode::Blend,
                cull_mode: None,
                ..default()
            })),
            Transform::from_xyz(0.0, 1.3, 1.8),
            MODES[0].1,
        ))
        .observe(report_click);
    // ANCHOR_END: veil
    println!("老雷：验货加一道纱幕——数字键 1234，给它换四种规矩。");
    println!("小棠：第 1 档，{}。", MODES[0].0);
}

fn report_click(click: On<Pointer<Click>>, names: Query<&Name>) {
    if let Ok(name) = names.get(click.entity) {
        println!("场记：{name}收到一点。");
    }
}

// ANCHOR: switch
/// 数字键换档：直接改纱幕身上的 Pickable 字段
fn switch_mode(keys: Res<ButtonInput<KeyCode>>, mut veil: Single<&mut Pickable, With<Veil>>) {
    for (i, key) in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4]
        .into_iter()
        .enumerate()
    {
        if keys.just_pressed(key) {
            let (words, mode) = MODES[i];
            **veil = mode;
            println!("小棠：第 {} 档，{words}。", i + 1);
        }
    }
}
// ANCHOR_END: switch
