//! Listing 23-2：读提货单——加载整份 Gltf，看看这个集装箱里装了什么

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, load_doc)
        .add_systems(Update, report_when_ready)
        .run();
}

// ANCHOR: handle
#[derive(Resource)]
struct PuppetDoc(Handle<Gltf>);

fn load_doc(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 不带 #SceneN 标签，加载的就是「整份」glTF——一个 Handle<Gltf>。
    // 它是文件的目录，能翻看里面有哪些场景、节点、动画，但本身还不能直接摆上台。
    commands.insert_resource(PuppetDoc(asset_server.load("models/puppet.gltf")));
}
// ANCHOR_END: handle

// ANCHOR: inspect
fn report_when_ready(doc: Res<PuppetDoc>, gltfs: Res<Assets<Gltf>>, mut done: Local<bool>) {
    if *done {
        return;
    }
    // 资源是异步加载的：加载完成前，Assets<Gltf> 里还查不到它，get 返回 None。
    let Some(gltf) = gltfs.get(&doc.0) else {
        return;
    };

    info!("场景 {} 个，动画 {} 段", gltf.scenes.len(), gltf.animations.len());
    info!("命名节点：{:?}", sorted_keys(gltf.named_nodes.keys()));
    info!("命名动画：{:?}", sorted_keys(gltf.named_animations.keys()));
    info!("命名材质：{:?}", sorted_keys(gltf.named_materials.keys()));
    *done = true;
}

// named_* 的键是 Box<str>，排个序好让每次输出一致
fn sorted_keys<'a>(keys: impl Iterator<Item = &'a Box<str>>) -> Vec<&'a str> {
    let mut names: Vec<&str> = keys.map(|s| &**s).collect();
    names.sort();
    names
}
// ANCHOR_END: inspect
