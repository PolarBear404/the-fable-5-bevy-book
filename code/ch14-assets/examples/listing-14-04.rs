//! Listing 14-4：库房广播——AssetEvent 的全生命周期：上架、到齐、回收、清位

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.08, 0.1)))
        .add_systems(Startup, hang_lantern)
        .add_systems(Update, (listen_broadcast, strike_the_prop))
        .run();
}

// ANCHOR: setup
/// 只记货号（AssetId 是 Copy 的纯编号）——记编号不算攥着提货单，不会拦住回收
#[derive(Resource)]
struct LanternId(AssetId<Image>);

fn hang_lantern(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let lantern: Handle<Image> = asset_server.load("props/lantern.png");
    commands.insert_resource(LanternId(lantern.id()));

    // 唯一一张提货单交给 Sprite：这件道具的命，从此系在这个实体身上
    commands.spawn(Sprite::from_image(lantern));
    println!("老顾：灯笼上场，提货单就一张，在道具自己手里。");
}
// ANCHOR_END: setup

// ANCHOR: listen
/// 听库房广播。AssetEvent 是 Message——第 7 章的 MessageReader 直接上
fn listen_broadcast(
    mut broadcasts: MessageReader<AssetEvent<Image>>,
    lantern: Res<LanternId>,
    mut frames: Local<u32>,
) {
    *frames += 1;
    for event in broadcasts.read() {
        match event {
            AssetEvent::Added { id } if *id == lantern.0 => {
                println!("广播：（第 {} 帧）灯笼上架。", *frames);
            }
            AssetEvent::LoadedWithDependencies { id } if *id == lantern.0 => {
                println!("广播：（第 {} 帧）灯笼连同全部配件到齐。", *frames);
            }
            AssetEvent::Unused { id } if *id == lantern.0 => {
                println!("广播：（第 {} 帧）最后一张提货单作废。", *frames);
            }
            AssetEvent::Removed { id } if *id == lantern.0 => {
                println!("广播：（第 {} 帧）货架清位，灯笼回炉。", *frames);
            }
            _ => {} // 别人的货、别的动静，不关咱们的事
        }
    }
}
// ANCHOR_END: listen

// ANCHOR: strike
/// 开演三秒后撤道具：实体销毁，它手里那张唯一的提货单随之销毁
fn strike_the_prop(
    mut commands: Commands,
    prop: Option<Single<Entity, With<Sprite>>>,
    time: Res<Time>,
) {
    let Some(prop) = prop else { return };
    if time.elapsed_secs() > 3.0 {
        println!("场务：这盏灯笼的戏拍完了，撤。");
        commands.entity(*prop).despawn();
    }
}
// ANCHOR_END: strike
