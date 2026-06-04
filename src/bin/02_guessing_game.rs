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
