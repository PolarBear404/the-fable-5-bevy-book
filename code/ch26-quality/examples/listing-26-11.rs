//! Listing 26-11：TAA 专场——M 键在 MSAA 开/关之间来回拨：
//! 开着，警告刷屏、接触阴影一地砂子；关掉，终端收声、砂子熔平

use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::pbr::ContactShadows;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, flip_msaa)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: camera
    // 故意的：挂了 TAA，却没把 Msaa 拨到 Off——出厂默认是 Sample4。
    // 接触阴影也在场，它满地的抖动采样噪点正等着 TAA 来抹
    commands.spawn((
        Camera3d::default(),
        TemporalAntiAliasing::default(),
        ContactShadows::default(),
        Transform::from_xyz(0.0, 1.7, 6.0).looking_at(Vec3::new(0.0, 0.6, 0.0), Vec3::Y),
    ));
    // ANCHOR_END: camera

    // 低角度的斜阳：擦着地皮照，缝隙里的小阴影全靠接触阴影补
    commands.spawn((
        DirectionalLight {
            illuminance: 18_000.0,
            shadow_maps_enabled: true,
            contact_shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, -0.9, -0.25)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.34, 0.36, 0.34),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 一面堂鼓贴地摆，鼓沿下那圈细缝是接触阴影的主场
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.8, 0.55))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.17, 0.13),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.28, 0.0),
    ));
    // 散一把鹅卵石：贴地小物越多，缝隙噪点越好认
    for (x, z, r) in [
        (-1.5, 0.8, 0.16),
        (-0.9, -0.9, 0.2),
        (1.2, 0.9, 0.14),
        (1.7, -0.5, 0.22),
        (0.6, 1.5, 0.12),
    ] {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(r))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.65, 0.62, 0.55),
                perceptual_roughness: 0.8,
                ..default()
            })),
            Transform::from_xyz(x, r, z),
        ));
    }

    println!("场记：TAA 挂上了，可 MSAA 没关——看你的终端，警告在刷屏。");
    println!("场记：M 键来回拨 MSAA：关，警告收声、砂子熔平；开，立马打回原形。");
}

// ANCHOR: flip
/// M 键在 Sample4 与 Off 之间来回拨——TAA 罢工与开工的分界线就这一个组件值
fn flip_msaa(keyboard: Res<ButtonInput<KeyCode>>, mut msaa: Single<&mut Msaa, With<Camera3d>>) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        **msaa = if **msaa == Msaa::Off {
            println!("场记：MSAA 回到 Sample4——警告又来了，砂子也回来了。");
            Msaa::Sample4
        } else {
            println!("场记：Msaa::Off——终端清净了，看鼓沿那圈砂子慢慢熔掉。");
            Msaa::Off
        };
    }
}
// ANCHOR_END: flip
