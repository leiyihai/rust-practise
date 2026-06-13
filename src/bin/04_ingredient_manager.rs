// 食材管理模拟器
// ================
// 任务：模拟厨房食材管理——食材可以"借用"但不能重复使用，练习所有权和借用。
//
// 提示：
// - 定义一个 Ingredient 结构体，包含 name 和 used 字段
// - 实现一个 Kitchen 结构体管理食材 Vec
// - 实现 borrow_ingredient(&self, name) -> Option<&Ingredient> —— 不可变借用
// - 实现 use_ingredient(&mut self, name) -> Result —— 可变借用，标记已使用
// - 返回引用时注意生命周期标注：fn get(&self, name: &str) -> Option<&Ingredient>
// - 尝试在持有不可变引用时调用可变方法，看编译器报什么错——理解借用规则
// - 思考：为什么 Rust 不允许同时存在可变引用和不可变引用？
//
// 运行方式：cargo run --bin 04_ingredient_manager
#[derive(Debug)]
struct Ingredient {
    name: String,
    used: bool,
}

impl Ingredient {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            used: false,
        }
    }
}

#[derive(Debug)]
struct Kitchen {
    items: Vec<Ingredient>,
}
impl Kitchen {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, name: &str) {
        self.items.push(Ingredient::new(name));
    }

    fn borrow_ingredient(&self, name: &str) -> Option<&Ingredient> {
        self.items.iter().find(|ing| ing.name == name && ing.used)
    }

    fn use_ingredient(&mut self, name: &str) -> Result<(), String> {
        match self.items.iter_mut().find(|ing| ing.name == name) {
            None => Err(format!("不存在食材：{}", name)),
            Some(ing) => {
                if ing.used {
                    Err(format!("{}已经被用完了", name))
                } else {
                    ing.used = true;
                    Ok(())
                }
            }
        }
    }
}

fn main() {
    let mut kitchen = Kitchen::new();
    kitchen.add("鸡蛋");
    kitchen.add("面粉");

    match kitchen.borrow_ingredient("鸡蛋") {
        Some(_) => println!("鸡蛋可以取用"),
        None => println!("鸡蛋不可用"),
    }

    if kitchen.use_ingredient("鸡蛋").is_ok() {
        println!("鸡蛋使用完毕");
    }

    let res = kitchen.borrow_ingredient("鸡蛋");
    println!("{:?}", res);
}
