//! Listing 4-3：精确取一个——按 Entity 用 get_mut，"恰好一个"用 single() 或 Single

use bevy::prelude::*;

#[derive(Component)]
struct Sheep;

#[derive(Component)]
struct Wolf;

#[derive(Component)]
struct Sheepdog;

#[derive(Component)]
struct Hunger(i32);

/// 牧羊犬心爱的羊——把 Entity 存进组件，就是"指向另一个实体的引用"
#[derive(Component)]
struct Favorite(Entity);

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_farm).add_systems(
        Update,
        (visit_favorite, watch_lone_wolf, wolf_census, nightfall).chain(),
    );

    app.update(); // 第 1 帧
    app.update(); // 第 2 帧
}

// ANCHOR: spawn
fn spawn_farm(mut commands: Commands) {
    commands.spawn((Name::new("小白"), Sheep, Hunger(6)));
    let curly = commands.spawn((Name::new("卷卷"), Sheep, Hunger(4))).id();
    commands.spawn((Name::new("阿黄"), Sheepdog, Favorite(curly)));
    commands.spawn((Name::new("灰背"), Wolf));
}
// ANCHOR_END: spawn

// ANCHOR: get_mut
/// 阿黄探望心爱的羊：按存好的 Entity 直取那一行
fn visit_favorite(
    dog: Single<(&Name, &Favorite)>,
    mut flock: Query<(&Name, &mut Hunger), With<Sheep>>,
) {
    let (dog_name, favorite) = *dog;
    match flock.get_mut(favorite.0) {
        Ok((sheep_name, mut hunger)) => {
            hunger.0 -= 1;
            println!("{dog_name} 给 {sheep_name} 留了口粮（饥饿降到 {}）", hunger.0);
        }
        Err(_) => println!("{dog_name} 到处找不到心爱的羊……"),
    }
}
// ANCHOR_END: get_mut

// ANCHOR: single_param
/// 狼恰好一只时，这个系统才运行；零只或多只都直接跳过
fn watch_lone_wolf(wolf: Single<&Name, With<Wolf>>) {
    println!("独狼 {} 在围栏外游荡，盯紧它", *wolf);
}
// ANCHOR_END: single_param

// ANCHOR: single_method
/// single() 把"是否恰好一只"交给你自己处理
fn wolf_census(wolves: Query<&Name, With<Wolf>>) {
    match wolves.single() {
        Ok(name) => println!("狼口普查：只有 {name} 一只"),
        Err(_) => println!("狼口普查：不是一只（{} 只），全员戒备！", wolves.iter().count()),
    }
}
// ANCHOR_END: single_method

// ANCHOR: nightfall
/// 入夜：心爱的羊被牧场主卖掉了，同时又来了一只狼（只发生一次）
fn nightfall(mut commands: Commands, dog: Single<&Favorite>, mut done: Local<bool>) {
    if *done {
        return;
    }
    *done = true;
    println!("〔夜里：卷卷被卖掉了，又一只狼摸了过来〕");
    commands.entity(dog.0).despawn();
    commands.spawn((Name::new("黑爪"), Wolf));
}
// ANCHOR_END: nightfall
