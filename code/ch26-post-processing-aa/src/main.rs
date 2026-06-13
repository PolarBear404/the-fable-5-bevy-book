//! 第 26 章画质开关面板示例。
//! 桌面运行：在 code/ 工作区执行 `cargo run -p ch26-post-processing-aa`。

use std::fmt::Write;

use bevy::{
    anti_alias::{
        contrast_adaptive_sharpening::ContrastAdaptiveSharpening,
        fxaa::{Fxaa, Sensitivity},
        smaa::{Smaa, SmaaPreset},
        taa::TemporalAntiAliasing,
    },
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    post_process::{
        bloom::Bloom,
        dof::{DepthOfField, DepthOfFieldMode},
        effect_stack::ChromaticAberration,
        motion_blur::MotionBlur,
    },
    prelude::*,
    render::view::Hdr,
};

fn main() {
    App::new()
        // ANCHOR: plugins
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: chapter_asset_path(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "第 26 章：画质开关面板".into(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.025, 0.028, 0.034)))
        .init_resource::<QualitySettings>()
        // ANCHOR_END: plugins
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                keyboard_quality_controls,
                apply_quality_settings,
                animate_stage,
                update_hud,
            )
                .chain(),
        )
        .run();
}

fn chapter_asset_path() -> String {
    std::env::var("BEVY_ASSET_ROOT")
        .unwrap_or_else(|_| format!("{}/assets", env!("CARGO_MANIFEST_DIR").replace('\\', "/")))
}

// ANCHOR: quality_settings
#[derive(Resource, Clone)]
struct QualitySettings {
    hdr: bool,
    tonemapping: Tonemapping,
    bloom: bool,
    bloom_preset: BloomPreset,
    depth_of_field: bool,
    motion_blur: bool,
    anti_aliasing: AntiAliasingMode,
    sharpening: bool,
    chromatic_aberration: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BloomPreset {
    Natural,
    Anamorphic,
    OldSchool,
    ScreenBlur,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AntiAliasingMode {
    Off,
    Msaa4,
    Fxaa,
    Smaa,
    Taa,
}

impl Default for QualitySettings {
    fn default() -> Self {
        match std::env::var("CH26_PRESET").as_deref() {
            Ok("raw") => Self {
                hdr: false,
                tonemapping: Tonemapping::None,
                bloom: false,
                bloom_preset: BloomPreset::Natural,
                depth_of_field: false,
                motion_blur: false,
                anti_aliasing: AntiAliasingMode::Off,
                sharpening: false,
                chromatic_aberration: false,
            },
            Ok("motion") => Self {
                hdr: true,
                tonemapping: Tonemapping::TonyMcMapface,
                bloom: true,
                bloom_preset: BloomPreset::Anamorphic,
                depth_of_field: false,
                motion_blur: true,
                anti_aliasing: AntiAliasingMode::Taa,
                sharpening: true,
                chromatic_aberration: false,
            },
            _ => Self {
                hdr: true,
                tonemapping: Tonemapping::TonyMcMapface,
                bloom: true,
                bloom_preset: BloomPreset::Natural,
                depth_of_field: true,
                motion_blur: false,
                anti_aliasing: AntiAliasingMode::Taa,
                sharpening: false,
                chromatic_aberration: false,
            },
        }
    }
}
// ANCHOR_END: quality_settings

impl BloomPreset {
    fn next(self) -> Self {
        match self {
            BloomPreset::Natural => BloomPreset::Anamorphic,
            BloomPreset::Anamorphic => BloomPreset::OldSchool,
            BloomPreset::OldSchool => BloomPreset::ScreenBlur,
            BloomPreset::ScreenBlur => BloomPreset::Natural,
        }
    }

    fn label(self) -> &'static str {
        match self {
            BloomPreset::Natural => "Natural",
            BloomPreset::Anamorphic => "Anamorphic",
            BloomPreset::OldSchool => "Old School",
            BloomPreset::ScreenBlur => "Screen Blur",
        }
    }

    fn component(self) -> Bloom {
        match self {
            BloomPreset::Natural => Bloom::NATURAL,
            BloomPreset::Anamorphic => Bloom::ANAMORPHIC,
            BloomPreset::OldSchool => Bloom::OLD_SCHOOL,
            BloomPreset::ScreenBlur => Bloom::SCREEN_BLUR,
        }
    }
}

impl AntiAliasingMode {
    fn next(self) -> Self {
        match self {
            AntiAliasingMode::Off => AntiAliasingMode::Msaa4,
            AntiAliasingMode::Msaa4 => AntiAliasingMode::Fxaa,
            AntiAliasingMode::Fxaa => AntiAliasingMode::Smaa,
            AntiAliasingMode::Smaa => AntiAliasingMode::Taa,
            AntiAliasingMode::Taa => AntiAliasingMode::Off,
        }
    }

    fn label(self) -> &'static str {
        match self {
            AntiAliasingMode::Off => "Off",
            AntiAliasingMode::Msaa4 => "MSAA 4x",
            AntiAliasingMode::Fxaa => "FXAA",
            AntiAliasingMode::Smaa => "SMAA High",
            AntiAliasingMode::Taa => "TAA",
        }
    }
}

fn next_tonemapping(tonemapping: Tonemapping) -> Tonemapping {
    match tonemapping {
        Tonemapping::None => Tonemapping::Reinhard,
        Tonemapping::Reinhard => Tonemapping::ReinhardLuminance,
        Tonemapping::ReinhardLuminance => Tonemapping::AcesFitted,
        Tonemapping::AcesFitted => Tonemapping::AgX,
        Tonemapping::AgX => Tonemapping::SomewhatBoringDisplayTransform,
        Tonemapping::SomewhatBoringDisplayTransform => Tonemapping::TonyMcMapface,
        Tonemapping::TonyMcMapface => Tonemapping::BlenderFilmic,
        Tonemapping::BlenderFilmic => Tonemapping::None,
    }
}

#[derive(Component)]
struct QualityCamera;

#[derive(Component)]
struct HudText;

#[derive(Component)]
struct Spins {
    speed: f32,
}

#[derive(Component)]
struct Slides {
    origin: Vec3,
    amplitude: f32,
    speed: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ANCHOR: scene
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.78, 0.82, 0.90),
        brightness: 95.0,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-0.35, -0.75, -0.45), Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            color: Color::srgb(0.45, 0.72, 1.0),
            intensity: 600_000.0,
            range: 14.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.8, 3.2, 2.6),
    ));

    let floor = materials.add(StandardMaterial {
        base_color: Color::srgb(0.11, 0.12, 0.13),
        perceptual_roughness: 0.88,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(18.0, 18.0))),
        MeshMaterial3d(floor),
    ));

    let matte = materials.add(StandardMaterial {
        base_color: Color::srgb(0.44, 0.52, 0.62),
        perceptual_roughness: 0.72,
        ..default()
    });
    let metal = materials.add(StandardMaterial {
        base_color: Color::srgb(0.76, 0.72, 0.64),
        metallic: 0.9,
        perceptual_roughness: 0.22,
        ..default()
    });
    let white = materials.add(StandardMaterial {
        base_color: Color::srgb(0.94, 0.94, 0.88),
        perceptual_roughness: 0.45,
        ..default()
    });
    let neon_blue = materials.add(StandardMaterial {
        base_color: Color::srgb(0.04, 0.12, 0.18),
        emissive: LinearRgba::rgb(0.0, 18.0, 45.0),
        ..default()
    });
    let neon_red = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.05, 0.04),
        emissive: LinearRgba::rgb(45.0, 6.0, 1.5),
        ..default()
    });

    let cube = meshes.add(Cuboid::new(0.65, 0.65, 0.65));
    let sphere = meshes.add(Sphere::new(0.34).mesh().uv(32, 16));
    let rail = meshes.add(Cuboid::new(0.035, 0.035, 4.8));
    let blade = meshes.add(Cuboid::new(0.18, 0.18, 2.6));

    for i in 0..18 {
        let x = -4.4 + i as f32 * 0.28;
        commands.spawn((
            Mesh3d(rail.clone()),
            MeshMaterial3d(white.clone()),
            Transform::from_xyz(x, 0.08, -1.5)
                .with_rotation(Quat::from_rotation_y(-0.72))
                .with_scale(Vec3::new(1.0, 1.0, 0.68)),
        ));
    }

    for i in 0..5 {
        let z = 1.4 - i as f32 * 1.25;
        let material = if i == 2 { metal.clone() } else { matte.clone() };
        commands.spawn((
            Mesh3d(cube.clone()),
            MeshMaterial3d(material),
            Transform::from_xyz(-0.7 + i as f32 * 0.35, 0.45, z)
                .with_rotation(Quat::from_rotation_y(i as f32 * 0.35)),
        ));
    }

    for i in 0..8 {
        commands.spawn((
            Mesh3d(sphere.clone()),
            MeshMaterial3d(if i % 2 == 0 {
                neon_blue.clone()
            } else {
                neon_red.clone()
            }),
            Transform::from_xyz(
                2.2 + (i % 4) as f32 * 0.55,
                0.6 + (i / 4) as f32 * 0.7,
                -1.1,
            ),
        ));
    }

    commands.spawn((
        Mesh3d(blade),
        MeshMaterial3d(neon_blue),
        Transform::from_xyz(2.6, 1.35, 1.0).with_rotation(Quat::from_rotation_z(0.35)),
        Spins { speed: 3.1 },
        Slides {
            origin: Vec3::new(2.6, 1.35, 1.0),
            amplitude: 1.2,
            speed: 1.4,
        },
    ));
    // ANCHOR_END: scene

    // ANCHOR: camera_bundle
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.025, 0.028, 0.034)),
            ..default()
        },
        Transform::from_xyz(0.0, 2.9, 7.4).looking_at(Vec3::new(0.0, 0.7, -0.9), Vec3::Y),
        QualityCamera,
        Hdr,
        Tonemapping::TonyMcMapface,
        DebandDither::Enabled,
        Bloom::NATURAL,
        DepthOfField {
            mode: DepthOfFieldMode::Gaussian,
            focal_distance: 7.1,
            aperture_f_stops: 0.18,
            max_circle_of_confusion_diameter: 28.0,
            max_depth: 16.0,
            ..default()
        },
        Msaa::Off,
        TemporalAntiAliasing::default(),
    ));
    // ANCHOR_END: camera_bundle

    // ANCHOR: ui
    let font = asset_server.load("fonts/book-sans-sc-regular.otf");
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(16),
            top: px(16),
            width: px(470),
            padding: UiRect::all(px(14)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.025, 0.028, 0.034, 0.82)),
        children![(
            HudText,
            Text::new(""),
            TextFont {
                font,
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.92, 0.95, 0.98)),
        )],
    ));
    // ANCHOR_END: ui

    println!("第 26 章画质开关面板：1 HDR，2 tonemapping，3 Bloom，4 DOF，5 MotionBlur，空格 AA。");
}

fn keyboard_quality_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<QualitySettings>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        settings.hdr = !settings.hdr;
        if !settings.hdr {
            settings.bloom = false;
        }
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        settings.tonemapping = next_tonemapping(settings.tonemapping);
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        settings.bloom = !settings.bloom;
        if settings.bloom {
            settings.hdr = true;
        }
    }
    if keyboard.just_pressed(KeyCode::KeyB) {
        settings.bloom_preset = settings.bloom_preset.next();
        settings.bloom = true;
        settings.hdr = true;
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        settings.depth_of_field = !settings.depth_of_field;
    }
    if keyboard.just_pressed(KeyCode::Digit5) {
        settings.motion_blur = !settings.motion_blur;
    }
    if keyboard.just_pressed(KeyCode::Space) {
        settings.anti_aliasing = settings.anti_aliasing.next();
    }
    if keyboard.just_pressed(KeyCode::Digit6) {
        settings.sharpening = !settings.sharpening;
    }
    if keyboard.just_pressed(KeyCode::Digit7) {
        settings.chromatic_aberration = !settings.chromatic_aberration;
    }
}

// ANCHOR: apply_settings
fn apply_quality_settings(
    mut commands: Commands,
    settings: Res<QualitySettings>,
    camera: Single<
        (
            Entity,
            &mut Msaa,
            &mut Tonemapping,
            Option<&Hdr>,
            Option<&Bloom>,
            Option<&DepthOfField>,
            Option<&MotionBlur>,
            Option<&Fxaa>,
            Option<&Smaa>,
            Option<&TemporalAntiAliasing>,
            Option<&ContrastAdaptiveSharpening>,
            Option<&ChromaticAberration>,
        ),
        With<QualityCamera>,
    >,
) {
    if !settings.is_changed() {
        return;
    }

    let (
        entity,
        mut msaa,
        mut tonemapping,
        has_hdr,
        has_bloom,
        has_depth_of_field,
        has_motion_blur,
        has_fxaa,
        has_smaa,
        has_taa,
        has_sharpening,
        has_chromatic_aberration,
    ) = camera.into_inner();

    let mut entity_commands = commands.entity(entity);
    if settings.hdr && has_hdr.is_none() {
        entity_commands.insert(Hdr);
    } else if !settings.hdr && has_hdr.is_some() {
        entity_commands.remove::<Hdr>();
    }

    *tonemapping = settings.tonemapping;

    if settings.bloom {
        entity_commands.insert(settings.bloom_preset.component());
    } else if has_bloom.is_some() {
        entity_commands.remove::<Bloom>();
    }

    if settings.depth_of_field {
        entity_commands.insert(DepthOfField {
            mode: DepthOfFieldMode::Gaussian,
            focal_distance: 7.1,
            aperture_f_stops: 0.18,
            max_circle_of_confusion_diameter: 28.0,
            max_depth: 16.0,
            ..default()
        });
    } else if has_depth_of_field.is_some() {
        entity_commands.remove::<DepthOfField>();
    }

    if settings.motion_blur {
        entity_commands.insert(MotionBlur {
            shutter_angle: 0.85,
            samples: 4,
        });
    } else if has_motion_blur.is_some() {
        entity_commands.remove::<MotionBlur>();
    }
    // ANCHOR_END: apply_settings

    // ANCHOR: aa_switch
    if has_fxaa.is_some() {
        entity_commands.remove::<Fxaa>();
    }
    if has_smaa.is_some() {
        entity_commands.remove::<Smaa>();
    }
    if has_taa.is_some() {
        entity_commands.remove::<TemporalAntiAliasing>();
    }

    match settings.anti_aliasing {
        AntiAliasingMode::Off => {
            *msaa = Msaa::Off;
        }
        AntiAliasingMode::Msaa4 => {
            *msaa = Msaa::Sample4;
        }
        AntiAliasingMode::Fxaa => {
            *msaa = Msaa::Off;
            entity_commands.insert(Fxaa {
                edge_threshold: Sensitivity::High,
                edge_threshold_min: Sensitivity::Medium,
                ..default()
            });
        }
        AntiAliasingMode::Smaa => {
            *msaa = Msaa::Off;
            entity_commands.insert(Smaa {
                preset: SmaaPreset::High,
            });
        }
        AntiAliasingMode::Taa => {
            *msaa = Msaa::Off;
            entity_commands.insert(TemporalAntiAliasing::default());
        }
    }
    // ANCHOR_END: aa_switch

    // ANCHOR: finishing_passes
    if settings.sharpening {
        entity_commands.insert(ContrastAdaptiveSharpening {
            sharpening_strength: 0.36,
            ..default()
        });
    } else if has_sharpening.is_some() {
        entity_commands.remove::<ContrastAdaptiveSharpening>();
    }

    if settings.chromatic_aberration {
        entity_commands.insert(ChromaticAberration {
            intensity: 0.012,
            max_samples: 8,
            ..default()
        });
    } else if has_chromatic_aberration.is_some() {
        entity_commands.remove::<ChromaticAberration>();
    }
    // ANCHOR_END: finishing_passes
}

fn animate_stage(
    time: Res<Time>,
    mut spinners: Query<(&mut Transform, Option<&Spins>, Option<&Slides>)>,
) {
    for (mut transform, spin, slide) in &mut spinners {
        if let Some(spin) = spin {
            transform.rotate_y(time.delta_secs() * spin.speed);
        }
        if let Some(slide) = slide {
            transform.translation = slide.origin
                + Vec3::X * (time.elapsed_secs() * slide.speed).sin() * slide.amplitude;
        }
    }
}

fn update_hud(settings: Res<QualitySettings>, mut text: Single<&mut Text, With<HudText>>) {
    if !settings.is_changed() {
        return;
    }

    let mut hud = String::new();
    let _ = writeln!(hud, "第 26 章：画质开关面板");
    let _ = writeln!(hud, "1  HDR: {}", on_off(settings.hdr));
    let _ = writeln!(hud, "2  Tonemapping: {:?}", settings.tonemapping);
    let _ = writeln!(
        hud,
        "3  Bloom: {}   B  preset: {}",
        on_off(settings.bloom),
        settings.bloom_preset.label()
    );
    let _ = writeln!(hud, "4  DepthOfField: {}", on_off(settings.depth_of_field));
    let _ = writeln!(hud, "5  MotionBlur: {}", on_off(settings.motion_blur));
    let _ = writeln!(
        hud,
        "Space  Anti-aliasing: {}",
        settings.anti_aliasing.label()
    );
    let _ = writeln!(hud, "6  CAS sharpening: {}", on_off(settings.sharpening));
    let _ = writeln!(
        hud,
        "7  Chromatic aberration: {}",
        on_off(settings.chromatic_aberration)
    );
    let _ = writeln!(hud);
    let _ = writeln!(
        hud,
        "提示：Bloom 会自动要求 HDR；TAA/FXAA/SMAA 会把 MSAA 关掉。"
    );
    text.0 = hud;
}

fn on_off(value: bool) -> &'static str {
    if value { "开" } else { "关" }
}
