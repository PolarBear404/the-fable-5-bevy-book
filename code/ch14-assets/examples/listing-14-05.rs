//! Listing 14-5：库里没这件货——加载失败的报错、Failed 状态与替身道具

use bevy::asset::{AssetLoadFailedEvent, LoadState};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.08, 0.1)))
        .add_systems(Startup, order_missing_prop)
        .add_systems(Update, hear_bad_news)
        .run();
}

// ANCHOR: order
#[derive(Resource)]
struct StaffOrder(Handle<Image>);

fn order_missing_prop(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    println!("老雷：下一场要一根如意杖。");
    println!("老顾：……单子先开上。（小声）库里好像没进过这件货。");
    // 这条路径不存在——load 照样立刻给单：库房只管开单，错要等跑腿回话
    commands.insert_resource(StaffOrder(asset_server.load("props/ruyi-staff.png")));
}
// ANCHOR_END: order

// ANCHOR: bad_news
/// 坏消息有两个口径：AssetLoadFailedEvent 广播（带原因），LoadState::Failed 状态牌
fn hear_bad_news(
    mut commands: Commands,
    mut bad_news: MessageReader<AssetLoadFailedEvent<Image>>,
    order: Option<Res<StaffOrder>>,
    asset_server: Res<AssetServer>,
) {
    let Some(order) = order else { return };

    for failure in bad_news.read() {
        println!("阿迅：回话——“{}”这件货取不来：{}", failure.path, failure.error);
    }

    if let LoadState::Failed(_) = asset_server.load_state(&order.0) {
        println!("老顾：状态牌也翻成“出事了”。戏不能停，先拿块灰布顶上！");
        // 替身道具：一块灰布（不需要任何资产文件）
        commands.spawn(Sprite::from_color(Color::srgb(0.4, 0.4, 0.42), Vec2::new(40.0, 160.0)));
        commands.remove_resource::<StaffOrder>();
    }
}
// ANCHOR_END: bad_news
