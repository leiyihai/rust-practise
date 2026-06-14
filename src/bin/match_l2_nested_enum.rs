// ============================================================
// 练习 8：嵌套枚举解构 match —— match_l2_nested_enum
// ============================================================
//
// 【题目需求】
// 实际开发中经常遇到嵌套枚举，如 Option<Result<T, E>>。
// 练习：给定一个 Option<Result<i32, String>>，用 match 逐层解构。
//
// 【具体要求】
// 1. 定义嵌套枚举变量：
//    let data: Option<Result<i32, String>> = Some(Ok(42));
// 2. 分别测试以下情况并打印对应信息：
//    - Some(Ok(n))    → "成功，数值：{n}"
//    - Some(Err(e))   → "失败，原因：{e}"
//    - None           → "无数据"
// 3. 尝试修改 data 为 Some(Err("...".to_string())) 和 None 验证
//
// 【考察点】
// - 嵌套模式解构：Some(Ok(n)) 一次性解构两层
// - 模式可以任意深度嵌套，编译器自动检查穷尽性
// - 实际场景：API 调用的缓存结果 Option<Result<Data, ApiError>>
// - 注意解构时变体顺序：模式匹配从上到下，兜底放最后
//
// 【代码骨架】
// fn main() {
//     let data: Option<Result<i32, String>> = Some(Ok(42));
//     // TODO: match 嵌套解构，同时处理 Some(Ok)、Some(Err)、None
//
//     let data2: Option<Result<i32, String>> = Some(Err("网络错误".to_string()));
//     // TODO: 验证 Err 分支
//
//     let data3: Option<Result<i32, String>> = None;
//     // TODO: 验证 None 分支
// }
//
// 【复习思考】
// Q: Some(Ok(n)) 和 Some(Err(e)) 能否用 Some(v) 然后内部再 match 替代？
// A: -------------------------------------------------
