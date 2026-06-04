# CLI 工具设计最佳实践

## 命令行参数的三个层次

```
myapp add --title "买菜" --amount 25.5
│     │   │                    │
│     │   └─ 标志 (flag/option) ─┘
│     └─── 子命令
└───────── 程序名
```

`std::env::args()` 给你的是 `Iterator<Item=String>`，你需要自己解析。实际项目中通常用 `clap` crate，它提供声明式的参数解析。

## 错误处理：用户看到的不应该是 Rust 的 panic 信息

```text
// 糟糕的用户体验
thread 'main' panicked at src/main.rs:42:17:
index out of bounds: the len is 0 but the index is 0

// 好的用户体验
错误：请提供命令。可用命令：add, list, remove
```

实践原则：
- 所有用户输入都是不可信的——解析失败给友好提示，不是 panic
- 用 `anyhow::Context` 给错误添加上下文
- 错误信息用中文（如果你的用户是中文用户）
- 退出码：0 成功，非 0 失败（`std::process::exit(1)`）

## 程序的模块化结构

即使是小 CLI 工具，也可以分层：

```text
src/
├── main.rs          # 入口：解析参数，调用逻辑
├── commands.rs      # 子命令的实现
├── model.rs         # 数据结构定义
└── storage.rs       # 读写文件的逻辑（如果有）
```

分层的好处：
- `main.rs` 很短，只做参数路由
- 业务逻辑可独立测试
- 新增子命令不影响已有代码

## 状态管理：是把数据放全局还是传参数

你的数据（比如记账列表）通常存在 `main` 里，然后通过参数传给函数：

```rust
fn add(records: &mut Vec<Record>, title: &str, amount: f64) {
    records.push(Record::new(title, amount));
}
```

避免全局可变状态——它让测试变得困难，也让借用检查变复杂。如果确实需要全局状态，考虑 `OnceLock` 或 `lazy_static`，但大部分情况不需要。

## 触类旁通：Unix 哲学

好的 CLI 工具遵循 Unix 哲学：
1. **做一件事，做好它** —— 一个工具一个职责
2. **期待文本流作为输入输出** —— 方便管道组合
3. **保持沉默** —— 成功时不输出垃圾，失败时说清楚
4. **stdin/stdout/stderr 各司其职** —— 数据走 stdout，日志/错误走 stderr

这些原则让 Rust 的 CLI 工具可以和 `grep`、`awk`、`jq` 等无缝协作。
