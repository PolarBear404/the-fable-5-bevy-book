use bevy::prelude::*;

// —— 组件定义 ——

/// 靶环的基础分值——每实体一份
#[derive(Component)]
struct Points(u32);

// —— 资源定义 ——

/// 场地难度——职业场记分翻倍
#[derive(Resource)]
struct Difficulty {
    pro: bool,
}

/// 记分规则：全场倍率，开场时由难度算出
#[derive(Resource)]
struct ScoreRules {
    multiplier: u32,
}

impl FromWorld for ScoreRules {
    fn from_world(world: &mut World) -> Self {
        let multiplier = if world.resource::<Difficulty>().pro { 2 } else { 1 };
        ScoreRules { multiplier }
    }
}

/// 计分板
#[derive(Resource, Default, PartialEq)]
struct Score(u32);

/// 双倍卡：在场即生效
#[derive(Resource)]
struct DoubleCard;

fn main() {
    let mut app = App::new();
    app.insert_resource(Difficulty { pro: true })
        .init_resource::<ScoreRules>() // 依赖 Difficulty，必须排在它之后
        .init_resource::<Score>() // Default：0 分开局
        .add_systems(Startup, setup_range)
        .add_systems(Update, (shoot, stall_keeper, scoreboard).chain());

    app.update(); // 第 1 枪
    app.update(); // 第 2 枪
    app.update(); // 第 3 枪
    app.update(); // 第 4 枪
}

// —— Startup：靶场开张 ——

fn setup_range(mut commands: Commands, rules: Res<ScoreRules>) {
    commands.spawn_batch([
        (Name::new("外环"), Points(2)),
        (Name::new("内环"), Points(5)),
        (Name::new("红心"), Points(10)),
    ]);
    println!("打靶场开张：职业场，全场 {} 倍记分", rules.multiplier);
}

// —— Update：一轮一枪 ——

/// 射手：命中得分 = 靶环基础分 × 全场倍率 ×（双倍卡在场再 ×2）
fn shoot(
    mut score: ResMut<Score>,
    rules: Res<ScoreRules>,
    card: Option<Res<DoubleCard>>,
    targets: Query<(&Name, &Points)>,
    mut round: Local<u32>,
) {
    *round += 1;
    // 剧本：前三枪依次瞄准三个靶环，第 4 枪脱靶
    let aim = ["外环", "红心", "内环"].get(*round as usize - 1).copied();
    let hit = aim.and_then(|aim| targets.iter().find(|(name, _)| name.as_str() == aim));

    let gained = match hit {
        Some((name, points)) => {
            let double = if card.is_some() { 2 } else { 1 };
            let gained = points.0 * rules.multiplier * double;
            let tag = if card.is_some() { "（双倍卡生效）" } else { "" };
            println!("第 {} 枪：命中 {name}，+{gained} 分{tag}", *round);
            gained
        }
        None => {
            println!("第 {} 枪：脱靶", *round);
            0
        }
    };
    let new_total = score.0 + gained;
    score.set_if_neq(Score(new_total));
}

/// 摊主：第 2 枪打中红心后递出双倍卡，用过一枪就收回
fn stall_keeper(mut commands: Commands, mut round: Local<u32>) {
    *round += 1;
    if *round == 2 {
        println!("摊主：红心都让你打中了，这张双倍卡送你！");
        commands.insert_resource(DoubleCard);
    }
    if *round == 3 {
        println!("摊主：双倍卡到期，收回了。");
        commands.remove_resource::<DoubleCard>();
    }
}

/// 记分牌：只在分数真的变了时刷新
fn scoreboard(score: Res<Score>) {
    if score.is_changed() {
        println!("记分牌 → {} 分", score.0);
    }
}
