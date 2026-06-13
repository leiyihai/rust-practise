// 猜数字小游戏（热/冷/温版）
// ==============================
// 任务：程序随机生成 1-100 的数字，用户有 5 次机会猜，每次猜错提示"热/冷/温"。
//
// 提示：
// - 需要在 Cargo.toml 中添加 rand 依赖：rand = "0.8"
// - 使用 rand::Rng 的 thread_rng().gen_range(1..=100) 生成随机数
// - 用 loop 或 for 控制最多 5 次尝试
// - 比较差值绝对值：<=10 为"热"，<=25 为"温"，>25 为"冷"
// - 用 match 处理 std::cmp::Ordering 判断大了还是小了
// - 输入解析失败时用 match 或 if let 处理 Result
// - 猜对时用 break 提前退出循环
//
// 运行方式：cargo run --bin 02_guessing_game
use rand::{Rng, thread_rng};
use std::cmp::Ordering;

fn main() {
    let secret_num = thread_rng().gen_range(1..=100);
    let can_guess_count = 10;
    let mut guess_count = 0;
    loop {
        if guess_count >= can_guess_count {
            println!("未猜中，游戏结束！");
            break;
        }

        println!("请输入你猜测的数字：");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("读取失败！");

        if let Ok(guess) = guess.trim().parse::<u32>() {
            if guess == secret_num {
                println!("猜对了!");
                break;
            } else if guess.abs_diff(secret_num) <= 10 {
                println!("热: {}", guess);
                cmp_num(guess, secret_num);
                guess_count += 1;
            } else if guess.abs_diff(secret_num) <= 25 {
                println!("温: {}", guess);
                cmp_num(guess, secret_num);
                guess_count += 1;
            } else if guess.abs_diff(secret_num) > 25 {
                println!("冷: {}", guess);
                cmp_num(guess, secret_num);
                guess_count += 1;
            }
            println!("剩余猜测次数: {}", can_guess_count - guess_count);
            println!();
        } else {
            println!("You need a number between 1 and 100.");
        }
    }
}

fn cmp_num(a: u32, b: u32) {
    match a.cmp(&b) {
        Ordering::Less => println!("偏小"),
        Ordering::Equal => println!("猜对了"),
        Ordering::Greater => println!("偏大"),
    }
}
