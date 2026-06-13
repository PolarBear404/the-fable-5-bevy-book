//! Listing 23-3（编译失败示例）：忘了贴标签——把整份 glTF 当场景用
//!
//! 不带 #SceneN 标签加载，拿到的是 Handle<Gltf>（整份文件的目录），
//! 而 SceneRoot 要的是 Handle<Scene>。类型对不上，编译器当场拦下。

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ANCHOR: wrong
    // 加载整份文件，拿到的是 Handle<Gltf>；直接塞给 SceneRoot——它要的是 Handle<Scene>
    let gltf: Handle<Gltf> = asset_server.load("models/puppet.gltf");
    commands.spawn(SceneRoot(gltf));
    // ANCHOR_END: wrong
}
