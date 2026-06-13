#[derive(Debug)]
struct Skill {
    name: String,
    cd: u32,
    origin_cd: u32,
}

#[derive(Debug)]
struct SkillBar {
    skills: Vec<Skill>,
}

impl SkillBar {
    fn new() -> SkillBar {
        SkillBar { skills: Vec::new() }
    }

    fn add_skill(&mut self, name: &str, init_cd: u32, origin_cd: u32) {
        match self.skills.iter_mut().find(|s| s.name == name ) {
            None => {self.skills.push(Skill { 
                name: name.to_string(), 
                cd: init_cd,
                origin_cd });},
            Some(skill) => {
                skill.cd = init_cd;
            },
        }
    }

    fn get_skill(&self, name: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.name == name)
    }

    fn use_skill(&mut self, name: &str) -> Result<(), String> {
        match self.skills.iter_mut().find(|s| s.name == name) {
            None => {
                Err(format!("{}技能不存在", name.to_string()))
            },
            Some(skill) => {
                if skill.cd > 0 {
                    return Err(format!("{}技能冷却中", name.to_string()))
                }else {
                    skill.cd = skill.origin_cd;
                    return Ok(())
                }
            },
        }
    }

    fn show_all(&self) {
        for skill in &self.skills {
            println!("{}:{}s", skill.name.to_string(), skill.cd);
        }
    }

    fn tick(&mut self) {
        for skill in &mut self.skills {
            if skill.cd > 0 {
                skill.cd -= 1;
            }
        }
    }

}

fn main() {
    let mut bar = SkillBar::new();

    // 火球原始冷却8帧，闪现原始冷却4帧
    bar.add_skill("火球术", 8, 8);
    bar.add_skill("闪现", 4, 4);
    bar.add_skill("沉默", 14, 14);
    bar.show_all();

    // 查询技能
    match bar.get_skill("无尽冬日") {
        Some(skill) => { println!("找到了技能:{}", skill.name) },
        None => { println!("未找到该技能") },
    }

    // 火球初始cd=8，不能直接放
    match bar.use_skill("火球术") {
        Ok(cd) => println!("释放成功，重置冷却{:?}", cd),
        Err(e) => println!("释放失败：{}", e),
    }

    // 手动流逝8帧，火球cd降到0
    for _ in 0..8 {
        bar.tick();
    }
    println!("\n流逝8帧后：");
    bar.show_all();

    // 现在可以释放，拿到它自己的init_cd=8
    match bar.use_skill("火球术") {
        Ok(reset_cd) => println!("✅火球释放成功，重置冷却为{:?}", reset_cd),
        Err(e) => println!("❌{}", e),
    }
    bar.show_all();

    // 测试闪现
    for _ in 0..4 {
        bar.tick();
    }
    match bar.use_skill("闪现") {
        Ok(cd) => println!("✅闪现释放成功，重置冷却{:?}", cd),
        Err(e) => println!("❌{}", e),
    }
}