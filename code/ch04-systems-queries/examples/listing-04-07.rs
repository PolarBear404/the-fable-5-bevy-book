//! Listing 4-7：ParamSet——让冲突的查询分时上岗

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
        .add_systems(Update, (extra_rations, evening_report).chain())
        .run();
}

fn spawn_flock(mut commands: Commands) {
    commands.spawn((Name::new("老灰"), Hunger(5), Wounded));
    commands.spawn((Name::new("小不点"), Hunger(8), Wounded, Young));
    commands.spawn((Name::new("卷卷"), Hunger(3)));
}

// ANCHOR: paramset
/// 伤员和幼崽各加一餐——ParamSet 化解了两个 &mut Hunger 查询的冲突
fn extra_rations(
    mut rations: ParamSet<(
        Query<(&Name, &mut Hunger), With<Wounded>>,
        Query<(&Name, &mut Hunger), With<Young>>,
    )>,
) {
    for (name, mut hunger) in rations.p0().iter_mut() {
        hunger.0 -= 1;
        println!("{name}（伤员）加餐");
    }
    for (name, mut hunger) in rations.p1().iter_mut() {
        hunger.0 -= 1;
        println!("{name}（幼崽）加餐");
    }
}
// ANCHOR_END: paramset

fn evening_report(flock: Query<(&Name, &Hunger)>) {
    println!("=== 晚间清点 ===");
    for (name, hunger) in &flock {
        println!("{name}  饥饿 {}", hunger.0);
    }
}
