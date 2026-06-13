// 小怪兽战斗游戏
// ================
// 任务：定义怪兽结构体和技能枚举，用 match 判断战斗结果。
//
// 提示：
// - 定义 Monster 结构体：name, hp, attack, defense 字段
// - 定义 Skill 枚举：FireBall, IceBeam, Heal, Thunder 等变体
// - 为 Skill 实现 damage() 方法，返回基础伤害值
// - 用 match 匹配不同技能，计算实际伤害（考虑属性相克）
// - 实现 Monster 的 attack_monster(&self, skill: &Skill, target: &mut Monster) 方法
// - 考虑使用 impl 块组织方法
// - 进阶：给技能枚举添加不同变体的附带数据（如 FireBall(u32) 表示威力）
// - 战斗循环：双方轮流攻击，直到一方 HP 归零
//
// 运行方式：cargo run --bin 05_monster_battle
fn main() {
    
}