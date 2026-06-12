//! Listing 11-9：艾达的随身柜台——#[derive(SystemParam)]

use bevy::app::ScheduleRunnerPlugin;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use std::time::Duration;

/// 常住户
#[derive(Component)]
struct Resident;

/// 镇库银两
#[derive(Resource)]
struct TownFunds(u32);

// ANCHOR: desk
/// 随身柜台：名册 + 镇库，一袋装走
#[derive(SystemParam)]
struct CensusDesk<'w, 's> {
    residents: Query<'w, 's, &'static Name, With<Resident>>,
    funds: ResMut<'w, TownFunds>,
}

impl CensusDesk<'_, '_> {
    /// 点名并收人头税：每户 1 枚
    fn collect(&mut self) -> u32 {
        let n = self.residents.iter().count() as u32;
        self.funds.0 += n;
        n
    }
}
// ANCHOR_END: desk

// ANCHOR: seasons
/// 春盘与秋盘：同一只柜台，原样复用
fn spring_census(mut desk: CensusDesk) {
    let n = desk.collect();
    println!("春盘：{n} 户造册，镇库现银 {} 枚。", desk.funds.0);
}

fn autumn_census(mut desk: CensusDesk, mut exit: MessageWriter<AppExit>) {
    let n = desk.collect();
    println!("秋盘：{n} 户造册，镇库现银 {} 枚。", desk.funds.0);
    exit.write(AppExit::Success);
}
// ANCHOR_END: seasons

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_millis(100),
        )))
        .insert_resource(TownFunds(100))
        .add_systems(Startup, settle_in)
        .add_systems(Update, (spring_census, autumn_census).chain())
        .run();
}

fn settle_in(mut commands: Commands) {
    commands.spawn((Resident, Name::new("罗兰")));
    commands.spawn((Resident, Name::new("老蔫儿")));
    commands.spawn((Resident, Name::new("杂货铺老板")));
}
