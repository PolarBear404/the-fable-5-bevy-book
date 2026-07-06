//! Listing 25-15：《上手验货》总场——指看、点名、双击归位、拖挪、装箱，
//! 外加一台完全由指针事件驱动的转台相机

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "上手验货".into(),
                // 这两个字段只在网页构建里生效（桌面下是空操作）：
                // 指定挂载的 <canvas>，并让画布随外框伸缩——20.7 的老朋友
                canvas: Some("#bevy-canvas".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MeshPickingPlugin)
        .insert_resource(Rig { yaw: 0.0, dist: 6.8 })
        .add_systems(Startup, setup)
        .add_systems(Update, seat_camera)
        .run();
}

/// 转台相机的两个旋钮：绕场角 + 机位距离
#[derive(Resource)]
struct Rig {
    yaw: f32,
    dist: f32,
}

/// 每件货的原座——双击归位时用
#[derive(Component)]
struct HomeSeat(Vec3);

/// 托盘记号
#[derive(Component)]
struct Tray;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    window: Single<Entity, With<Window>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.8, 6.8).looking_at(Vec3::new(0.0, 0.8, 0.0), Vec3::Y),
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
        Mesh3d(meshes.add(Plane3d::default().mesh().size(14.0, 14.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.40, 0.40, 0.42),
            perceptual_roughness: 0.9,
            ..default()
        })),
        // 台面不抢戏：拾取上完全隐身，拖地板等于拖空处——转台的手感来源
        Pickable::IGNORE,
    ));

    // 朱漆托盘（25.8 的成交台）
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
    let tray_base = tray_paint.clone();
    commands
        .spawn((
            Name::new("托盘"),
            Tray,
            Mesh3d(meshes.add(Cuboid::new(2.2, 0.16, 2.2))),
            MeshMaterial3d(tray_paint),
            Transform::from_xyz(2.8, 0.58, 0.8),
        ))
        .observe(
            move |enter: On<Pointer<DragEnter>>,
                  mut coats: Query<&mut MeshMaterial3d<StandardMaterial>>| {
                if let Ok(mut coat) = coats.get_mut(enter.entity) {
                    coat.0 = tray_glow.clone();
                }
            },
        )
        .observe(
            move |leave: On<Pointer<DragLeave>>,
                  mut coats: Query<&mut MeshMaterial3d<StandardMaterial>>| {
                if let Ok(mut coat) = coats.get_mut(leave.entity) {
                    coat.0 = tray_base.clone();
                }
            },
        )
        .observe(
            |drop: On<Pointer<DragDrop>>,
             names: Query<&Name>,
             trays: Query<&Transform, With<Tray>>,
             mut wares: Query<&mut Transform, Without<Tray>>| {
                if let (Ok(name), Ok(tray_seat), Ok(mut ware_seat)) = (
                    names.get(drop.dropped),
                    trays.get(drop.entity),
                    wares.get_mut(drop.dropped),
                ) {
                    println!("托盘：成交——{name}装箱。");
                    ware_seat.translation = tray_seat.translation + Vec3::new(0.0, 0.72, 0.0);
                }
            },
        );

    // 高亮漆一罐，全场共用
    let spotlight = materials.add(StandardMaterial {
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
            Transform::from_xyz(-2.2, 1.0, 0.0),
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
            Transform::from_xyz(-4.0, 0.98, 1.2),
        ),
    ];
    // ANCHOR: wares
    for (name, mesh, paint, seat) in wares {
        let original = materials.add(paint);
        commands
            .spawn((
                Name::new(name),
                Mesh3d(mesh),
                MeshMaterial3d(original.clone()),
                seat,
                HomeSeat(seat.translation),
            ))
            // 指看：高亮迎客，走了还原
            .observe(recolor_on::<Pointer<Over>>(spotlight.clone()))
            .observe(recolor_on::<Pointer<Out>>(original))
            // 点名与双击归位：一只观察者里分流
            .observe(
                |click: On<Pointer<Click>>,
                 names: Query<&Name>,
                 mut wares: Query<(&mut Transform, &HomeSeat)>| {
                    let Ok(name) = names.get(click.entity) else {
                        return;
                    };
                    if click.count == 2 {
                        if let Ok((mut seat, home)) = wares.get_mut(click.entity) {
                            seat.translation = home.0;
                            println!("场记：{name}双击归位。");
                        }
                    } else {
                        println!("场记：{name}收到一点。");
                    }
                },
            )
            // 拖挪三件套：拖起让路、跟手挪、落定复牌
            .observe(|start: On<Pointer<DragStart>>, mut commands: Commands| {
                commands.entity(start.entity).insert(Pickable::IGNORE);
            })
            .observe(drag_ware)
            .observe(|end: On<Pointer<DragEnd>>, mut commands: Commands| {
                commands.entity(end.entity).insert(Pickable::default());
            });
    }
    // ANCHOR_END: wares

    // ANCHOR: window_rig
    // 转台相机：全部输入走窗口实体的指针事件——
    // 拖空处转场（起头必须是窗口自己，拖货冒泡来的不算），滚轮推拉
    let stage_door = *window;
    commands
        .entity(stage_door)
        .observe(move |drag: On<Pointer<Drag>>, mut rig: ResMut<Rig>| {
            if drag.original_event_target() == stage_door {
                rig.yaw -= drag.delta.x * 0.008;
            }
        })
        .observe(|scroll: On<Pointer<Scroll>>, mut rig: ResMut<Rig>| {
            rig.dist = (rig.dist - scroll.y * 0.6).clamp(3.0, 12.0);
        });
    // ANCHOR_END: window_rig

    println!("老雷：《上手验货》总场——指看点名，双击归位，拖挪装箱；");
    println!("老雷：拖空处转台，滚轮推拉。陆掌柜，请。");
}

/// 25.2 的观察者工厂，原样搬来
fn recolor_on<E: EntityEvent>(
    paint: Handle<StandardMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut coats| {
        if let Ok(mut coat) = coats.get_mut(event.event_target()) {
            coat.0 = paint.clone();
        }
    }
}

/// 拖挪：沿相机的右轴与上轴走，转台转到哪边都跟手
fn drag_ware(
    drag: On<Pointer<Drag>>,
    mut wares: Query<&mut Transform, Without<Camera3d>>,
    camera: Single<&Transform, With<Camera3d>>,
) {
    if let Ok(mut seat) = wares.get_mut(drag.entity) {
        let step = (*camera.right() * drag.delta.x - *camera.up() * drag.delta.y) * 0.008;
        seat.translation += step;
    }
}

// ANCHOR: seat_camera
/// 旋钮变了才重摆机位：绕着台心 (0, 0.8, 0) 的圆轨
fn seat_camera(rig: Res<Rig>, mut camera: Single<&mut Transform, With<Camera3d>>) {
    if !rig.is_changed() {
        return;
    }
    let center = Vec3::new(0.0, 0.8, 0.0);
    let seat = center + Vec3::new(rig.dist * rig.yaw.sin(), 2.0, rig.dist * rig.yaw.cos());
    **camera = Transform::from_translation(seat).looking_at(center, Vec3::Y);
}
// ANCHOR_END: seat_camera
