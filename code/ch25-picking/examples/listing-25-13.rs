//! Listing 25-13：现成的自由脚架——FreeCamera 在画廊里飞

use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin, FreeCameraState};
use bevy::prelude::*;

fn main() {
    App::new()
        // 控制器插件单独请：它不在 DefaultPlugins 里
        .add_plugins((DefaultPlugins, FreeCameraPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (tune, report_seat))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.8, 6.4).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
        // 挂上这个组件，相机就归控制器管了；FreeCameraState 是它的
        // required component（第 3 章的机制），不用自己写
        FreeCamera {
            walk_speed: 3.0,
            run_speed: 9.0,
            friction: 25.0,
            ..default()
        },
    ));
    // ANCHOR_END: camera
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
    let wares: [(Handle<Mesh>, StandardMaterial, Transform); 3] = [
        (
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
            meshes.add(Cuboid::new(0.95, 0.95, 0.95)),
            StandardMaterial {
                base_color: Color::srgb(0.62, 0.11, 0.08),
                perceptual_roughness: 0.35,
                ..default()
            },
            Transform::from_xyz(2.0, 0.98, 0.0),
        ),
    ];
    for (mesh, paint, seat) in wares {
        commands.spawn((Mesh3d(mesh), MeshMaterial3d(materials.add(paint)), seat));
    }
    println!("老雷：脚架换自由云台——右键按住转镜头，WASD 走、EQ 升降、Shift 跑。");
    println!("小棠：Z/X 拨灵敏度，C/V 拨刹车，空格报机位。");
}

// ANCHOR: tune
/// 运行时拨参：FreeCamera 的字段全是普通数据，改就生效
fn tune(keys: Res<ButtonInput<KeyCode>>, mut camera: Single<&mut FreeCamera>) {
    if keys.just_pressed(KeyCode::KeyZ) {
        camera.sensitivity = (camera.sensitivity - 0.05).max(0.05);
        println!("小棠：灵敏度拨到 {:.2}。", camera.sensitivity);
    }
    if keys.just_pressed(KeyCode::KeyX) {
        camera.sensitivity += 0.05;
        println!("小棠：灵敏度拨到 {:.2}。", camera.sensitivity);
    }
    if keys.just_pressed(KeyCode::KeyC) {
        camera.friction = (camera.friction - 10.0).max(5.0);
        println!("小棠：刹车拨到 {:.0}——越小滑得越远。", camera.friction);
    }
    if keys.just_pressed(KeyCode::KeyV) {
        camera.friction += 10.0;
        println!("小棠：刹车拨到 {:.0}。", camera.friction);
    }
}
// ANCHOR_END: tune

// ANCHOR: report
/// 空格报机位：位置读 Transform，速度读控制器的状态组件
fn report_seat(
    keys: Res<ButtonInput<KeyCode>>,
    camera: Single<(&Transform, &FreeCameraState)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let (seat, state) = *camera;
        println!(
            "场记：机位 ({:.1}, {:.1}, {:.1})，时速 {:.1}。",
            seat.translation.x,
            seat.translation.y,
            seat.translation.z,
            state.velocity.length()
        );
    }
}
// ANCHOR_END: report
