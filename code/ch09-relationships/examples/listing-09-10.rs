//! Listing 9-10：allow_self_referential——随队郎中自己给自己诊治

use bevy::prelude::*;

// ANCHOR: derive
/// 关系源：这个人正由谁诊治（挂在病人身上）
#[derive(Component)]
#[relationship(relationship_target = Patients, allow_self_referential)]
struct TreatedBy(Entity);

/// 关系目标：郎中手头的病人名单（引擎自动维护）
#[derive(Component)]
#[relationship_target(relationship = TreatedBy)]
struct Patients(Vec<Entity>);
// ANCHOR_END: derive

fn main() {
    App::new()
        // 特意挂上 LogPlugin：要是引擎有意见，警告逃不过我们的眼睛
        .add_plugins(bevy::log::LogPlugin::default())
        .add_systems(Startup, injuries)
        .add_systems(Update, ward_round)
        .run();
}

// ANCHOR: injuries
fn injuries(mut commands: Commands) {
    let doctor = commands.spawn(Name::new("随队郎中")).id();
    // 罗兰磕破了手，交给郎中
    commands.spawn((Name::new("罗兰"), TreatedBy(doctor)));
    // 郎中自己也扭了脚——自己诊治自己，关系指向自己
    commands.entity(doctor).insert(TreatedBy(doctor));
}
// ANCHOR_END: injuries

// ANCHOR: ward_round
/// 巡诊：每位郎中报出手头的病人
fn ward_round(doctors: Query<(&Name, &Patients)>, names: Query<&Name>) {
    for (doctor_name, patients) in &doctors {
        let list: Vec<&str> = patients
            .iter()
            .map(|patient| names.get(patient).unwrap().as_str())
            .collect();
        println!("{doctor_name} 正在诊治：{}", list.join("、"));
    }
}
// ANCHOR_END: ward_round
