#[derive(Debug, Clone)]
// 怪兽属性枚举，用来做克制判定
enum Element {
    Fire,
    Ice,
    Thunder,
}

#[derive(Debug)]
struct Monster {
    name: String,
    hp: u32,
    attack: u32,
    defense: u32,
    element: Element, // 自身属性
}

#[derive(Debug)]
enum Skill {
    FireBall(u32),
    IceBeam(u32),
    Thunder(u32),
    Heal(u32),
}

impl Skill {
    // 获取技能基础威力
    fn base_power(&self) -> u32 {
        match self {
            Skill::FireBall(p) => *p,
            Skill::IceBeam(p) => *p,
            Skill::Thunder(p) => *p,
            Skill::Heal(p) => *p,
        }
    }

    // 获取技能所属属性，治疗无属性
    fn get_element(&self) -> Option<Element> {
        match self {
            Skill::FireBall(_) => Some(Element::Fire),
            Skill::IceBeam(_) => Some(Element::Ice),
            Skill::Thunder(_) => Some(Element::Thunder),
            Skill::Heal(_) => None,
        }
    }
}

impl Monster {
    fn new(name: &str, hp: u32, attack: u32, defense: u32, element: Element) -> Self {
        Monster {
            name: name.to_string(),
            hp,
            attack,
            defense,
            element,
        }
    }

    // 判断是否存活
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    // 攻击另一只怪兽
    fn attack_monster(&self, skill: &Skill, target: &mut Monster) {
        // 治疗技能：给自己回血，不攻击
        if let Skill::Heal(heal_val) = skill {
            println!("{} 使用治疗术，回复{}血量！", self.name, heal_val);
            // 直接修改自身血量，注意self不可变，这里不能改，调整思路：治疗单独方法演示
            return;
        }

        let skill_power = skill.base_power() as f64;
        let atk = self.attack as f64;
        let def = target.defense as f64;

        // 属性克制倍率
        let mut rate = 1.0;
        if let Some(skill_elem) = skill.get_element() {
            rate = match (&skill_elem, &target.element) {
                (Element::Fire, Element::Ice) => 1.5,
                (Element::Thunder, Element::Fire) => 1.5,
                (Element::Ice, Element::Thunder) => 1.5,
                _ => 1.0,
            };
        }

        // 伤害计算公式，最低0点伤害
        let mut damage = (atk + skill_power) * rate - def;
        if damage < 0.0 {
            damage = 0.0;
        }
        let damage_u32 = damage.round() as u32;

        // 扣血，血量不低于0
        if target.hp > damage_u32 {
            target.hp -= damage_u32;
        } else {
            target.hp = 0;
        }

        println!(
            "{} 释放技能，对 {} 造成 {} 点伤害，对方剩余血量：{}",
            self.name, target.name, damage_u32, target.hp
        );
    }

    // 单独治疗自己
    fn heal_self(&mut self, heal_skill: &Skill) {
        if let Skill::Heal(val) = heal_skill {
            self.hp += val;
            println!(
                "{} 治疗自己，恢复{}血量，当前血量：{}",
                self.name, val, self.hp
            );
        }
    }
}

fn main() {
    // 创建两只怪兽
    let mut monster1 = Monster::new("小火龙", 100, 25, 8, Element::Fire);
    let mut monster2 = Monster::new("冰精灵", 90, 22, 10, Element::Ice);

    // 预设技能
    let fire_skill = Skill::FireBall(30);
    let ice_skill = Skill::IceBeam(28);
    let heal_skill = Skill::Heal(25);

    println!("====== 战斗开始 ======");
    let mut round = 1;

    // 轮流攻击循环
    while monster1.is_alive() && monster2.is_alive() {
        println!("\n【第{}回合】", round);
        round += 1;

        // 怪兽1攻击怪兽2
        monster1.attack_monster(&fire_skill, &mut monster2);
        if !monster2.is_alive() {
            println!("\n{} 战败！{} 胜利！", monster2.name, monster1.name);
            break;
        }

        // 怪兽2攻击怪兽1
        monster2.attack_monster(&ice_skill, &mut monster1);
        if !monster1.is_alive() {
            println!("\n{} 战败！{} 胜利！", monster1.name, monster2.name);
            break;
        }

        // 演示一次治疗
        if round == 3 {
            monster1.heal_self(&heal_skill);
        }
    }
}
