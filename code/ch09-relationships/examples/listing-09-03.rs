//! Listing 9-3：两桩闯祸——ChildOf 指向自己、指向不存在的实体

use bevy::prelude::*;

fn main() {
    App::new()
        // 引擎的警告走日志系统，挂上 LogPlugin 才看得见
        .add_plugins(bevy::log::LogPlugin::default())
        .add_systems(Startup, mishaps)
        .add_systems(Update, aftermath)
        .run();
}

// ANCHOR: mishaps
fn mishaps(mut commands: Commands) {
    // 闯祸一：让青篷车自己拉自己
    let wagon = commands.spawn(Name::new("青篷车")).id();
    commands.entity(wagon).insert(ChildOf(wagon));

    // 闯祸二：登上一辆已经报废的马车
    let scrapped = commands.spawn(Name::new("报废马车")).id();
    commands.entity(scrapped).despawn();
    commands.spawn((Name::new("倒霉旅人"), ChildOf(scrapped)));
}
// ANCHOR_END: mishaps

// ANCHOR: aftermath
fn aftermath(everyone: Query<(&Name, Option<&ChildOf>)>) {
    println!("== 事后清点 ==");
    for (name, child_of) in &everyone {
        match child_of {
            Some(child_of) => println!("{name}：挂在 {} 下面", child_of.parent()),
            None => println!("{name}：不挂在任何东西下面"),
        }
    }
}
// ANCHOR_END: aftermath
