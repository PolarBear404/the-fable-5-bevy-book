//! Listing 4-6：两个 &mut 查询盯上同一列——编译通过，启动即 panic（B0001）

use bevy::prelude::*;

#[derive(Component)]
struct Hunger(i32);

/// 受伤标记
#[derive(Component)]
struct Wounded;

/// 幼崽标记
#[derive(Component)]
struct Young;

fn main() {
    App::new()
        .add_systems(Startup, spawn_flock)
        .add_systems(Update, extra_rations)
        .run();
}

fn spawn_flock(mut commands: Commands) {
    commands.spawn((Name::new("老灰"), Hunger(5), Wounded));
    commands.spawn((Name::new("小不点"), Hunger(8), Wounded, Young));
    commands.spawn((Name::new("卷卷"), Hunger(3)));
}

// ANCHOR: conflict
/// 伤员和幼崽各加一餐——能编译，但一启动就 panic
fn extra_rations(
    mut wounded: Query<(&Name, &mut Hunger), With<Wounded>>,
    mut young: Query<(&Name, &mut Hunger), With<Young>>,
) {
    for (name, mut hunger) in &mut wounded {
        hunger.0 -= 1;
        println!("{name}（伤员）加餐");
    }
    for (name, mut hunger) in &mut young {
        hunger.0 -= 1;
        println!("{name}（幼崽）加餐");
    }
}
// ANCHOR_END: conflict
