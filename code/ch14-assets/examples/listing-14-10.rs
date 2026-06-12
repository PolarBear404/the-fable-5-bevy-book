//! Listing 14-10：缝进戏服的场记板——embedded_asset 把文件嵌进可执行文件

use bevy::asset::embedded_asset;
use bevy::prelude::*;

// ANCHOR: plugin
struct EmbeddedClapperPlugin;

impl Plugin for EmbeddedClapperPlugin {
    fn build(&self, app: &mut App) {
        // 把 examples/embedded/clapper.png 的字节编进二进制。
        // 第二个参数是路径前缀：本文件不在 src/ 下，得告诉宏从 examples 起算
        embedded_asset!(app, "examples", "embedded/clapper.png");
    }
}
// ANCHOR_END: plugin

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedClapperPlugin))
        .insert_resource(ClearColor(Color::srgb(0.06, 0.06, 0.08)))
        .add_systems(Startup, raise_clapper)
        .run();
}

// ANCHOR: load
fn raise_clapper(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // 嵌入资产住在 embedded:// 来源下；“crate 名”是本例的可执行名 listing_14_10
    let clapper: Handle<Image> =
        asset_server.load("embedded://listing_14_10/embedded/clapper.png");
    commands.spawn(Sprite::from_image(clapper));

    println!("老顾：场记板不走库房——缝在戏服里，走到哪带到哪。");
    println!("老顾：就算把 assets/ 整个搬空，这块板子照样举得起来。");
}
// ANCHOR_END: load
