// 名字反转器
// ============
// 任务：读取用户输入的名字，反转并打印，同时输出名字长度。
//
// 提示：
// - 使用 std::io::stdin() 读取输入
// - String 的 .chars() 返回字符迭代器
// - .rev() 可以反转迭代器
// - .collect() 可以将迭代器收集为 String
// - .len() 获取字符串字节长度，.chars().count() 获取字符数
// - 注意：read_line 读取的内容包含换行符，需要用 .trim() 去除
// - 思考：变量在传给 println! 后还能继续使用吗？（所有权移动）
//
// 运行方式：cargo run --bin 01_name_reverser

fn main() {
    let mut input = String::new();
    println!("请输入字符：");
    std::io::stdin().read_line(&mut input).expect("读取失败");
    let reverse_name = name_reverser(&input);
    println!("输出字符：{}", reverse_name);
}

fn name_reverser(name: &str) -> String {
    name.chars().rev().collect()
}
