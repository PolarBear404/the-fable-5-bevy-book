//! Listing 25-3：指到哪件亮哪件——Over/Out 一对进出事件

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}

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
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(12.0, 12.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.40, 0.42),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // 三件货共用一支「请看这件」的高亮漆——句柄克隆即可，账本里只此一罐
    let spotlight_paint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.98, 0.86, 0.35),
        emissive: LinearRgba::rgb(0.35, 0.25, 0.05),
        ..default()
    });

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
        // 原装漆入库拿好句柄——退场时要原样换回去
        let original = materials.add(paint);
        commands
            .spawn((Name::new(name), Mesh3d(mesh), MeshMaterial3d(original.clone()), seat))
            .observe(recolor_on::<Pointer<Over>>(spotlight_paint.clone()))
            .observe(recolor_on::<Pointer<Out>>(original))
            .observe(announce_over)
            .observe(announce_out);
    }
    println!("老雷：陆掌柜先过眼——指到哪件，哪件替你亮起来。");
}
// ANCHOR_END: setup

// ANCHOR: factory
/// 观察者工厂：造一个「事件 E 一响就给目标换上 paint」的闭包。
/// 同一份逻辑，Over 配高亮漆、Out 配原装漆，各造一个挂上去
fn recolor_on<E: EntityEvent>(
    paint: Handle<StandardMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut coats| {
        if let Ok(mut coat) = coats.get_mut(event.event_target()) {
            coat.0 = paint.clone();
        }
    }
}
// ANCHOR_END: factory

// ANCHOR: announce
fn announce_over(over: On<Pointer<Over>>, names: Query<&Name>) {
    if let Ok(name) = names.get(over.entity) {
        println!("场记：指针进了{name}的地界（命中深度 {:.2}）。", over.hit.depth);
    }
}

fn announce_out(out: On<Pointer<Out>>, names: Query<&Name>) {
    if let Ok(name) = names.get(out.entity) {
        println!("场记：指针离了{name}。");
    }
}
// ANCHOR_END: announce
