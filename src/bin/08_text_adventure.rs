// 文字冒险游戏
// =============
// 任务：用结构体、枚举、模式匹配实现一个可交互的文字冒险游戏。
//
// 提示：
// - 定义 Room 结构体：name, description, exits（用 HashMap 或 Vec）
// - 定义 Action 枚举：Go(String), Look, Take(String), Use(String), Quit
// - 用 match 解析用户输入并分发到不同逻辑
// - 维护 Player 状态（当前位置、背包）
// - 游戏主循环：打印当前房间 -> 读取输入 -> 匹配动作 -> 更新状态
// - 思考：如果房间很多，如何组织代码？模块系统 mod 怎么用？
//
// 运行方式：cargo run --bin 08_text_adventure
