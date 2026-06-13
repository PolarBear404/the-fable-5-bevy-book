//! Listing 23-4：点谁的名——场景展开后，按节点名字找到实体并给它挂东西

use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.52, 0.55, 0.62)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 加载场景，并给这个实体挂一个观察者：场景一展开就触发
    commands
        .spawn(SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("models/puppet.gltf"),
        )))
        .observe(give_flag);

    stage(&mut commands, &mut meshes, &mut materials);
}

// ANCHOR: give_flag
/// glTF 里每个节点的名字，加载后会变成实体上的 Name 组件。
/// 场景展开后遍历子孙，按名字找到 "ArmRight"，往它手里塞一面小旗。
fn give_flag(
    ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    names: Query<&Name>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in children.iter_descendants(ready.entity) {
        let Ok(name) = names.get(entity) else {
            continue;
        };
        if name.as_str() == "ArmRight" {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.55, 0.4, 0.04))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.95, 0.82, 0.25),
                    emissive: LinearRgba::rgb(0.6, 0.5, 0.1),
                    ..default()
                })),
                // 小旗钉在手的下端；做 ArmRight 的子实体，它就跟着这条胳膊走
                Transform::from_xyz(0.0, -1.05, 0.25),
                ChildOf(entity),
            ));
            info!("找到了 ArmRight，把小旗挂上了");
        }
    }
}
// ANCHOR_END: give_flag

// 台子：地面、主光、机位、一点环境光
fn stage(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    commands.insert_resource(GlobalAmbientLight {
        brightness: 200.0,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.28, 0.30, 0.33),
            perceptual_roughness: 0.95,
            ..default()
        })),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.5, -0.9, -0.4), Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::new(0.0, 1.9, 0.0), Vec3::Y),
    ));
}
