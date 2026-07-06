//! Listing 25-10：亲手放线——MeshRayCast 一线串起糖葫芦，连隐身的也点名

use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::prelude::*;

fn main() {
    App::new()
        // 只用 MeshRayCast 这个系统参数，不需要 MeshPickingPlugin——
        // 射线检测是独立工具，拾取管线只是它的老主顾
        .add_plugins(DefaultPlugins)
        .init_resource::<SeeAll>()
        .add_systems(Startup, setup)
        .add_systems(Update, (cast_line, switch_visibility))
        .run();
}

/// V 键拨的档：要不要连隐身实体一起串
#[derive(Resource, Default)]
struct SeeAll(bool);

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
        Name::new("台面"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.40, 0.42),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // ANCHOR: lineup
    // 三件货沿视线一列纵队（相机望向 (0, 0.8, 0)，这条视线就是糖葫芦签子）：
    // 瞄画面正中放线，一线从前到后依次穿过盏身、锣的下环带、盒腹
    commands.spawn((
        Name::new("琉璃盏"),
        Mesh3d(meshes.add(Sphere::new(0.55))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.35, 0.62, 0.60, 0.35),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.089,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.30, 1.6),
    ));
    commands.spawn((
        Name::new("鎏金锣"),
        Mesh3d(meshes.add(Torus::new(0.28, 0.72))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.82, 0.55),
            metallic: 1.0,
            perceptual_roughness: 0.25,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.30, 0.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
    // 备货藏在队尾：Hidden 了，画面上看不见
    commands.spawn((
        Name::new("备用漆盒"),
        Mesh3d(meshes.add(Cuboid::new(0.95, 0.95, 0.95))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.62, 0.11, 0.08),
            perceptual_roughness: 0.35,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.48, -1.8),
        Visibility::Hidden,
    ));
    // ANCHOR_END: lineup
    println!("小棠：这回不用拾取管线，自己放线——左键放线串货，V 键换「看不看隐身」。");
}

// ANCHOR: cast
/// 左键放线：光标反算出 Ray3d，穿透模式串出沿线全部命中
fn cast_line(
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut ray_cast: MeshRayCast,
    names: Query<&Name>,
    see_all: Res<SeeAll>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let (camera, camera_seat) = *camera;
    // 17.3 的老三步，3D 版：反算出的不再是一个点，而是一条射线
    let Ok(ray) = camera.viewport_to_world(camera_seat, cursor) else {
        return;
    };

    let settings = MeshRayCastSettings {
        // VisibleInView：只串镜头里看得见的；Any：仓库里藏的也串
        visibility: if see_all.0 { RayCastVisibility::Any } else { RayCastVisibility::VisibleInView },
        // 每个实体都参与（这里不筛人）
        filter: &|_entity| true,
        // 永不早退：穿透到底，有几件串几件
        early_exit_test: &|_entity| false,
    };
    let hits = ray_cast.cast_ray(ray, &settings);

    if hits.is_empty() {
        println!("场记：这一线放空了。");
        return;
    }
    for (entity, hit) in hits {
        let name = names.get(*entity).map(|n| n.as_str()).unwrap_or("无名");
        println!("场记：{:.2} 米处串到{name}。", hit.distance);
    }
    println!("——共 {} 件。", hits.len());
}
// ANCHOR_END: cast

fn switch_visibility(keys: Res<ButtonInput<KeyCode>>, mut see_all: ResMut<SeeAll>) {
    if keys.just_pressed(KeyCode::KeyV) {
        see_all.0 = !see_all.0;
        println!(
            "小棠：换档——{}。",
            if see_all.0 { "Any：隐身的也串" } else { "VisibleInView：只串看得见的" }
        );
    }
}
