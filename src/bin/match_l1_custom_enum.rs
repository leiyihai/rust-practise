// ============================================================
// 练习 5：自定义枚举解构 match —— match_l1_custom_enum
// ============================================================
//
// 【题目需求】
// 定义一个极简技能枚举，match 分发不同行为。
// 基础版：变体不带数据；进阶版：变体带威力数值并解构取出。
//
// 【具体要求】
// 1. 定义枚举 Skill { Fire, Ice, Heal }
// 2. 创建技能变量，match 判断：
//    - Fire → 打印"火球伤害"
//    - Ice  → 打印"冰霜伤害"
//    - Heal → 打印"回复生命"
// 3. 【进阶变种】修改枚举为 Fire(u32), Ice(u32), Heal(u32)
//    在 match 分支中解构取出威力数值，打印"火球伤害：{power}"
//
// 【考察点】
// - 自定义枚举的定义和使用
// - 枚举变体带数据时的解构绑定
// - 实际开发中背包、技能、账户状态等全是这套写法
//
// 【代码骨架 - 基础版】
// enum Skill {
//     Fire,
//     Ice,
//     Heal,
// }

// fn main() {
//     let skill = Skill::Fire;
//     match skill {
//         Skill::Fire => println!("火球伤害"),
//         Skill::Ice => println!("冰霜伤害"),
//         Skill::Heal => println!("回复生命"),
//     }
// }
//
// 【代码骨架 - 进阶版】
enum Skill {
    Fire(u32),
    Ice(u32),
    Heal(u32),
}

fn main() {
    let skill = Skill::Fire(120);
    match skill {
        Skill::Fire(dmg) => println!("火球伤害：{}", dmg),
        Skill::Ice(dmg) => println!("冰霜伤害：{}", dmg),
        Skill::Heal(hp) => println!("回复生命：{}", hp),
    }
}
//
// 【复习思考】
// Q: 上面几道题中，哪些不能删掉 _ 兜底分支？为什么？
// A: 上面哪有几道题？
