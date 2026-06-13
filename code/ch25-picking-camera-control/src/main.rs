//! Listing 25-6：点选 / 拖拽 3D 物体 + FreeCamera。
//! 桌面 `cargo run` 开窗口；编成 wasm 后渲染进网页 <canvas>。

use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    picking::pointer::PointerButton,
    prelude::*,
};

fn main() {
    App::new()
        // ANCHOR: web_window
        // 仅 Web 生效：桌面平台忽略 canvas / fit_canvas_to_parent。
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "第 25 章：Picking 与相机控制".into(),
                resolution: (1280, 720).into(),
                canvas: Some("#bevy-ch25".into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        // ANCHOR_END: web_window
        // ANCHOR: plugins
        .add_plugins((MeshPickingPlugin, FreeCameraPlugin))
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ..default()
        })
        // ANCHOR_END: plugins
        .insert_resource(ClearColor(Color::srgb(0.035, 0.042, 0.055)))
        .insert_resource(SelectedActor::default())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource, Default)]
struct SelectedActor {
    entity: Option<Entity>,
}

#[derive(Component)]
struct ActorName(&'static str);

#[derive(Component, Clone)]
struct ActorMaterials {
    idle: Handle<StandardMaterial>,
    hover: Handle<StandardMaterial>,
    pressed: Handle<StandardMaterial>,
    selected: Handle<StandardMaterial>,
}

#[derive(Component)]
struct StatusLine;

type ActorQuery<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static ActorName,
        &'static ActorMaterials,
        &'static mut MeshMaterial3d<StandardMaterial>,
    ),
>;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.13, 0.14, 0.16),
        perceptual_roughness: 0.85,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(18.0, 18.0))),
        MeshMaterial3d(floor_mat),
        Pickable::IGNORE,
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 6500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.45, -0.75, -0.35), Vec3::Y),
    ));
    commands.spawn((
        PointLight {
            intensity: 900_000.0,
            range: 18.0,
            color: Color::srgb(1.0, 0.82, 0.62),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.5, 4.0, 3.5),
    ));

    // ANCHOR: actor_materials
    let actor_materials = ActorMaterials {
        idle: materials.add(Color::srgb(0.45, 0.58, 0.78)),
        hover: materials.add(Color::srgb(0.28, 0.86, 0.86)),
        pressed: materials.add(Color::srgb(1.0, 0.76, 0.30)),
        selected: materials.add(Color::srgb(0.98, 0.38, 0.48)),
    };
    // ANCHOR_END: actor_materials

    let cube = meshes.add(Cuboid::new(1.1, 1.1, 1.1));
    let sphere = meshes.add(Sphere::new(0.65).mesh().uv(32, 18));
    let capsule = meshes.add(Capsule3d::new(0.42, 0.9));

    // ANCHOR: spawn_actors
    spawn_actor(
        &mut commands,
        "箱笼",
        cube,
        Vec3::new(-2.2, 0.65, 0.0),
        actor_materials.clone(),
    );
    spawn_actor(
        &mut commands,
        "玉球",
        sphere,
        Vec3::new(0.0, 0.75, -0.4),
        actor_materials.clone(),
    );
    spawn_actor(
        &mut commands,
        "立柱",
        capsule,
        Vec3::new(2.2, 0.8, 0.1),
        actor_materials,
    );
    // ANCHOR_END: spawn_actors

    // ANCHOR: camera
    commands.spawn((
        Camera3d::default(),
        MeshPickingCamera,
        Transform::from_xyz(0.0, 2.6, 7.2).looking_at(Vec3::new(0.0, 0.7, 0.0), Vec3::Y),
        FreeCamera {
            mouse_key_cursor_grab: MouseButton::Right,
            keyboard_key_toggle_cursor_grab: KeyCode::KeyM,
            walk_speed: 4.0,
            run_speed: 12.0,
            friction: 35.0,
            ..default()
        },
    ));
    // ANCHOR_END: camera

    let ui_font = asset_server.load("fonts/book-sans-sc-regular.otf");
    spawn_hud(&mut commands, ui_font);

    println!("左键点选 / 拖拽物体；右键拖动视角；WASD/QE 移动；Shift 加速；滚轮调速度。");
}

fn spawn_actor(
    commands: &mut Commands,
    name: &'static str,
    mesh: Handle<Mesh>,
    position: Vec3,
    materials: ActorMaterials,
) {
    commands
        .spawn((
            Name::new(name),
            ActorName(name),
            Mesh3d(mesh),
            MeshMaterial3d(materials.idle.clone()),
            Transform::from_translation(position),
            Pickable::default(),
            materials,
        ))
        .observe(on_actor_over)
        .observe(on_actor_out)
        .observe(on_actor_press)
        .observe(on_actor_release)
        .observe(on_actor_click)
        .observe(on_actor_drag);
}

fn spawn_hud(commands: &mut Commands, font: Handle<Font>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(16),
            top: px(16),
            padding: UiRect::all(px(14)),
            flex_direction: FlexDirection::Column,
            row_gap: px(6),
            ..default()
        },
        BackgroundColor(Color::srgba(0.04, 0.05, 0.07, 0.78)),
        Pickable::IGNORE,
        children![
            (
                Text::new("左键点选 / 拖拽物体；右键拖动视角"),
                TextFont {
                    font: font.clone(),
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Pickable::IGNORE,
            ),
            (
                StatusLine,
                Text::new("尚未选中"),
                TextFont {
                    font: font.clone(),
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.72, 0.86, 1.0)),
                Pickable::IGNORE,
            ),
        ],
    ));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: px(18),
                bottom: px(18),
                padding: UiRect::axes(px(16), px(10)),
                border: UiRect::all(px(1)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.09, 0.10, 0.13)),
            BorderColor::all(Color::srgb(0.28, 0.86, 0.86)),
        ))
        .observe(clear_selection)
        .with_child((
            Text::new("清空选中"),
            TextFont {
                font,
                font_size: 21.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Pickable::IGNORE,
        ));
}

fn set_actor_material(
    entity: Entity,
    choose: impl FnOnce(&ActorMaterials) -> Handle<StandardMaterial>,
    actors: &mut ActorQuery,
) {
    if let Ok((_, _, materials, mut material)) = actors.get_mut(entity) {
        material.0 = choose(materials);
    }
}

fn on_actor_over(
    event: On<Pointer<Over>>,
    selected: Res<SelectedActor>,
    mut actors: ActorQuery,
) {
    if selected.entity == Some(event.entity) {
        return;
    }
    set_actor_material(event.entity, |m| m.hover.clone(), &mut actors);
}

fn on_actor_out(
    event: On<Pointer<Out>>,
    selected: Res<SelectedActor>,
    mut actors: ActorQuery,
) {
    if selected.entity == Some(event.entity) {
        return;
    }
    set_actor_material(event.entity, |m| m.idle.clone(), &mut actors);
}

fn on_actor_press(event: On<Pointer<Press>>, mut actors: ActorQuery) {
    if event.button == PointerButton::Primary {
        set_actor_material(event.entity, |m| m.pressed.clone(), &mut actors);
    }
}

fn on_actor_release(
    event: On<Pointer<Release>>,
    selected: Res<SelectedActor>,
    mut actors: ActorQuery,
) {
    if event.button != PointerButton::Primary {
        return;
    }
    if selected.entity == Some(event.entity) {
        set_actor_material(event.entity, |m| m.selected.clone(), &mut actors);
    } else {
        set_actor_material(event.entity, |m| m.hover.clone(), &mut actors);
    }
}

// ANCHOR: click
fn on_actor_click(
    mut event: On<Pointer<Click>>,
    mut selected: ResMut<SelectedActor>,
    mut actors: ActorQuery,
    mut status: Single<&mut Text, With<StatusLine>>,
) {
    if event.button != PointerButton::Primary {
        return;
    }
    selected.entity = Some(event.entity);
    let mut picked_name = "未知物体";
    for (entity, name, materials, mut material) in &mut actors {
        if entity == event.entity {
            material.0 = materials.selected.clone();
            picked_name = name.0;
        } else {
            material.0 = materials.idle.clone();
        }
    }
    status.0 = format!("已选中：{picked_name}。继续拖拽可把它沿地面推走。");
    event.propagate(false);
}
// ANCHOR_END: click

// ANCHOR: drag
fn on_actor_drag(
    mut event: On<Pointer<Drag>>,
    camera: Single<&Transform, (With<Camera3d>, With<FreeCamera>, Without<ActorName>)>,
    mut transforms: Query<&mut Transform, (With<ActorName>, Without<FreeCamera>)>,
    mut status: Single<&mut Text, With<StatusLine>>,
) {
    if event.button != PointerButton::Primary {
        return;
    }
    let Ok(mut transform) = transforms.get_mut(event.entity) else {
        return;
    };

    let right = Vec3::new(camera.right().x, 0.0, camera.right().z).normalize_or_zero();
    let forward = Vec3::new(camera.forward().x, 0.0, camera.forward().z).normalize_or_zero();
    transform.translation += (right * event.delta.x - forward * event.delta.y) * 0.01;
    transform.translation.x = transform.translation.x.clamp(-4.2, 4.2);
    transform.translation.z = transform.translation.z.clamp(-3.2, 3.2);

    status.0 = "正在拖拽物体：Pointer<Drag>::delta 是屏幕像素，这里投到相机的地面方向。".into();
    event.propagate(false);
}
// ANCHOR_END: drag

fn clear_selection(
    mut event: On<Pointer<Click>>,
    mut selected: ResMut<SelectedActor>,
    mut actors: ActorQuery,
    mut status: Single<&mut Text, With<StatusLine>>,
) {
    if event.button != PointerButton::Primary {
        return;
    }
    selected.entity = None;
    for (_, _, materials, mut material) in &mut actors {
        material.0 = materials.idle.clone();
    }
    status.0 = "尚未选中".into();
    event.propagate(false);
}
