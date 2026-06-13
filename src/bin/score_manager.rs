#[derive(Debug)]
struct Score{
    name: String,
    score: u32,
}

#[derive(Debug)]
struct Class{
    scores: Vec<Score>,
}

impl Class {
    fn new() -> Class{
        Class { scores: Vec::new() }
    }

    fn add(&mut self, stu_name: &str, score: u32){
        match self.scores.iter_mut().find(|stu| stu.name == stu_name) {
            Some(s) => {
                s.score = score;
            }
            None => self.scores.push(Score { name: stu_name.to_string(), score }),
        }
    }

    fn get_score(&self, stu_name: &str) -> Option<&Score>{
        self.scores.iter().find(|stu|stu.name == stu_name)
    }

    fn modify_score(&mut self, stu_name: &str, new_score:u32) -> Result<u32, String>{
        match self.scores.iter_mut().find(|stu|stu.name == stu_name) {
            None => Err(format!("学生「{}」不存在", stu_name)),
            Some(stu) => {
                stu.score = new_score;
                Ok(stu.score)
            }
        }
    }

    fn delete_student(&mut self, stu_name: &str) -> Result<(), String>{
         match self.scores.iter_mut().find(|stu|stu.name == stu_name) {
            None => Err(format!("{}同学不存在", stu_name)),
            Some(_) => {
                self.scores.retain(|stu|stu.name != stu_name);
                Ok(())
            }
         }
    }

    fn list_all(&self){
        for stu in &self.scores{
            println!("{}: {}", stu.name, stu.score);
        }
    }
}

fn main() {
    let mut class = Class::new();
    class.add("小明", 90);
    class.add("小红", 85);
    class.add("小黑", 99);
    class.add("小白", 60);

    match class.get_score("小明") {
        Some(s) =>{
            println!("{}的分数是：{}分", s.name, s.score);
        }
        None =>{}
    }

    match class.modify_score("小红", 98) {
        Err(e) => println!("修改失败：{}", e),
        Ok(s) => println!("分数修改成：{}分", s),
    }

    match class.delete_student("小明") {
        Err(e) => println!("删除失败：{}", e),
        Ok(_) => println!("小明已删除"),
    }

    match class.get_score("小明") {
        Some(s) => print!("{}的分数是：{}", s.name, s.score),
        None => println!("小明不存在！"),
    }

    class.list_all();
}