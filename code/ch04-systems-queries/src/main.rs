use bevy::prelude::*;

// —— 组件定义 ——

/// 饥饿度；新生羊羔默认 6
#[derive(Component)]
struct Hunger(i32);

impl Default for Hunger {
    fn default() -> Self {
        Hunger(6)
    }
}

/// 生命值；默认 50
#[derive(Component)]
struct Health(i32);

impl Default for Health {
    fn default() -> Self {
        Health(50)
    }
}

/// 羊：天生有饥饿度和生命值
#[derive(Component, Default)]
#[require(Hunger, Health)]
struct Sheep;

/// 狼标记
#[derive(Component)]
struct Wolf;

/// 牧羊犬标记
#[derive(Component)]
struct Sheepdog;

/// 受伤标记
#[derive(Component)]
struct Wounded;

fn main() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_farm).add_systems(
        Update,
        (
            sunrise,
            graze,
            recovery,
            wolf_attack,
            sheepdog_guard,
            lambing,
            register,
            nightfall,
        )
            .chain(),
    );

    app.update(); // 第 1 天
    app.update(); // 第 2 天
    app.update(); // 第 3 天
}

// —— Startup：牧场开张 ——

fn spawn_farm(mut commands: Commands) {
    commands.spawn((Name::new("小白"), Sheep, Hunger(6), Health(60)));
    commands.spawn((Name::new("小黑"), Sheep, Hunger(6), Health(55)));
    commands.spawn((Name::new("卷卷"), Sheep, Hunger(4), Health(40)));
    commands.spawn((Name::new("阿黄"), Sheepdog, Health(70)));
    commands.spawn((Name::new("灰背"), Wolf, Health(80)));
}

// —— Update：牧场的一天 ——

/// 报晓：Local 记着今天是第几天
fn sunrise(mut day: Local<u32>) {
    *day += 1;
    println!("—— 第 {} 天 ——", *day);
}

/// 吃草：每只羊饥饿 -1
fn graze(mut flock: Query<&mut Hunger, With<Sheep>>) {
    for mut hunger in &mut flock {
        hunger.0 -= 1;
    }
}

/// 静养：伤员每天恢复 15 点生命，痊愈后摘掉 Wounded
fn recovery(
    mut commands: Commands,
    mut wounded: Query<(Entity, &Name, &mut Health), With<Wounded>>,
) {
    for (entity, name, mut health) in &mut wounded {
        health.0 += 15;
        if health.0 >= 40 {
            println!("{name} 伤愈归队（生命 {}）", health.0);
            commands.entity(entity).remove::<Wounded>();
        } else {
            println!("{name} 还在羊圈静养（生命 {}）", health.0);
        }
    }
}

/// 狼袭击：挑生命值最低的健康羊下口
fn wolf_attack(
    mut commands: Commands,
    wolves: Query<&Name, With<Wolf>>,
    mut flock: Query<(Entity, &Name, &mut Health), (With<Sheep>, Without<Wounded>)>,
) {
    for wolf_name in &wolves {
        // iter() 给出只读视图，先找目标，再用 get_mut 精确下口
        let Some((victim, _, _)) = flock.iter().min_by_key(|(_, _, health)| health.0) else {
            return;
        };
        let Ok((_, sheep_name, mut health)) = flock.get_mut(victim) else {
            return;
        };
        health.0 -= 25;
        println!("{wolf_name} 咬伤了 {sheep_name}！（生命 {}）", health.0);
        commands.entity(victim).insert(Wounded);
    }
}

/// 牧羊犬护场：发现新伤员就把狼赶出牧场
fn sheepdog_guard(
    mut commands: Commands,
    dog: Single<&Name, With<Sheepdog>>,
    newly_wounded: Query<&Name, Added<Wounded>>,
    wolves: Query<(Entity, &Name), With<Wolf>>,
) {
    for sheep_name in &newly_wounded {
        println!("{} 发现 {sheep_name} 受了伤！", *dog);
        for (wolf, wolf_name) in &wolves {
            println!("{} 冲出去，把 {wolf_name} 赶出了牧场", *dog);
            commands.entity(wolf).despawn();
        }
    }
}

/// 添丁：全群吃饱（饥饿 ≤ 3）的那天，羊羔出生——只发生一次
fn lambing(mut commands: Commands, flock: Query<&Hunger, With<Sheep>>, mut born: Local<bool>) {
    if !*born && flock.iter().all(|hunger| hunger.0 <= 3) {
        commands.spawn((Name::new("羊羔"), Sheep));
        *born = true;
    }
}

/// 登记员：名册添上新来的羊
fn register(newcomers: Query<&Name, Added<Sheep>>) {
    for name in &newcomers {
        println!("名册新增：{name}");
    }
}

/// 夜幕点名：羊、狼、牧羊犬一并清点
fn nightfall(
    animals: Query<
        (&Name, Option<&Hunger>, &Health, Has<Wounded>),
        Or<(With<Sheep>, With<Wolf>, With<Sheepdog>)>,
    >,
) {
    println!("· 夜幕点名");
    for (name, hunger, health, is_wounded) in &animals {
        let hunger_text = match hunger {
            Some(hunger) => format!("饥饿 {}", hunger.0),
            None => "饥饿 —".to_string(),
        };
        let tag = if is_wounded { "（伤）" } else { "" };
        println!("  {name}  {hunger_text}  生命 {}{tag}", health.0);
    }
}
