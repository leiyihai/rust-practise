// ============================================================
// 练习 6：Result 枚举 match（必练）—— match_l2_result
// ============================================================
//
// 【题目需求】
// Result 是 Rust 中处理"可能失败"的标准枚举，与 Option 同等重要。
// 给定一个 Result<String, String>，用 match 分别处理成功和失败两种情况。
//
// 【具体要求】
// 1. 定义 Result 变量：
//    - let res: Result<String, String> = Ok("操作成功".to_string());
//    - 再试一个 let res: Result<String, String> = Err("文件不存在".to_string());
// 2. match res，解构 Ok 和 Err 变体
// 3. Ok(val) → 打印"成功：{val}"
// 4. Err(e)  → 打印"错误：{e}"
// 5. 额外挑战：match 表达式返回不同的字符串，赋值给变量并打印
//
// 【考察点】
// - Result 有两个泛型参数：Ok(T) 成功值、Err(E) 错误值
// - 与 Option 对比：Option 无值不说明原因，Result 的 Err 可以携带错误信息
// - Result 也是编译器强制穷尽的枚举，必须处理两种可能
// - 实际开发中文件读写、网络请求、解析等全用 Result
//
// 【代码骨架】
fn main() {
    let res: Result<String, String> = Ok("数据加载完成".to_string());
    let res_err: Result<String, String> = Err("文件不存在".to_string());
    find_res(res);
    find_res(res_err);

    let res2: Result<String, String> = Err("连接超时".to_string());
    find_res(res2);
}

fn find_res(r: Result<String, String>) {
    match r {
        Ok(val) => {
            println!("成功：{val}")
        }
        Err(e) => {
            println!("错误：{e}")
        }
    }
}
//
// 【复习思考】
// Q: Result<T, E> 和 Option<T> 有什么本质区别？什么时候用哪个？
// A: 不知道本质区别。一般查找返回用Opeiont<T>,增删改用Result<T, E>
// Q: match Result 时能否省略 Err 分支？
// A: 不能。match是穷举的。
