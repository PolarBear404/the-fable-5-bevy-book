//! Listing 25-11：2D 这边的规矩——sprite 拾取要挂牌，还认像素不认框

use bevy::sprite::{SpritePickingMode, SpritePickingSettings};
use bevy::prelude::*;

fn main() {
    App::new()
        // SpritePickingPlugin 随 DefaultPlugins 自动就位，不用手请
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (enroll, switch_mode, muffle))
        .run();
}

#[derive(Component)]
struct Ayan;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Single<Entity, With<Window>>) {
    commands.spawn(Camera2d);

    // ANCHOR: sprites
    // 阿燕站前排（z=1），灯笼吊后排（z=0）——两张图的包围盒故意叠着
    commands
        .spawn((
            Name::new("阿燕"),
            Ayan,
            Sprite::from_image(asset_server.load("sprites/ayan-still.png")),
            // 32×40 的像素画放大十倍，别糊：Sprite 的近邻采样 ch15 讲过，
            // 这里偷懒用默认线性采样，实验只看拾取不看清晰度
            Transform::from_xyz(-80.0, 0.0, 1.0).with_scale(Vec3::splat(10.0)),
        ))
        .observe(report_click);
    commands
        .spawn((
            Name::new("灯笼"),
            Sprite::from_image(asset_server.load("sprites/lantern.png")),
            Transform::from_xyz(40.0, 60.0, 0.0).with_scale(Vec3::splat(10.0)),
        ))
        .observe(report_click);
    // ANCHOR_END: sprites

    // 台口兜底（25.5 的手法）。注意判定用 original_event_target：
    // 冒泡来的账单起头是货，直接命中的起头才是台口自己——只有后者算落空
    let stage_door = *window;
    commands
        .entity(stage_door)
        .observe(move |click: On<Pointer<Click>>| {
            if click.original_event_target() == stage_door {
                println!("场记：这一点落空了。");
            }
        });

    println!("老雷：2D 铺面开张——先点点看，再按 P 给两件挂牌。");
    println!("小棠：M 键换「认像素/认框」，B 键给阿燕换吸音档。");
}

fn report_click(click: On<Pointer<Click>>, names: Query<&Name>) {
    if let Ok(name) = names.get(click.entity) {
        println!("场记：{name}收到一点。");
    }
}

// ANCHOR: enroll
/// P 键挂牌：sprite 后端只认挂了 Pickable 的实体——mesh 那边的默认全收，
/// 到 2D 这边成了默认全不收
fn enroll(
    keys: Res<ButtonInput<KeyCode>>,
    sprites: Query<Entity, With<Sprite>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        for sprite in &sprites {
            commands.entity(sprite).insert(Pickable::default());
        }
        println!("小棠：牌挂上了——两件都在册。");
    }
}
// ANCHOR_END: enroll

// ANCHOR: mode
/// M 键换判定：AlphaThreshold 逐像素查不透明度，BoundingBox 只看矩形
fn switch_mode(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<SpritePickingSettings>) {
    if keys.just_pressed(KeyCode::KeyM) {
        settings.picking_mode = match settings.picking_mode {
            SpritePickingMode::AlphaThreshold(_) => {
                println!("小棠：换「认框」——包围盒里全算数。");
                SpritePickingMode::BoundingBox
            }
            SpritePickingMode::BoundingBox => {
                println!("小棠：换「认像素」——alpha 过 0.1 才算摸到。");
                SpritePickingMode::AlphaThreshold(0.1)
            }
        };
    }
}
// ANCHOR_END: mode

/// B 键给阿燕换吸音档——挡下家、自己不收（25.6 那档在 2D 后端下的对照）
fn muffle(
    keys: Res<ButtonInput<KeyCode>>,
    mut ayan: Single<&mut Pickable, (With<Ayan>, With<Sprite>)>,
) {
    if keys.just_pressed(KeyCode::KeyB) {
        **ayan = Pickable { should_block_lower: true, is_hoverable: false };
        println!("小棠：阿燕换吸音档——挡下家，自己不收。");
    }
}
