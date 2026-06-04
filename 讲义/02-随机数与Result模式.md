# 随机数与 Result 模式

## rand crate 的内部原理

`rand::thread_rng()` 返回的是一个**线程局部的 CSPRNG**（密码学安全伪随机数生成器），底层使用操作系统的熵源（Linux 的 `/dev/urandom`，Windows 的 `BCryptGenRandom`）。

```rust
use rand::Rng;
let n: u32 = rand::thread_rng().gen_range(1..=100);
```

`1..=100` 是 `RangeInclusive<i32>` 类型。Rust 用类型系统确保范围是有效的——你没法传空范围。

## Result：Rust 的"错误不抛异常"

Rust 没有 try-catch。错误通过返回值传播：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

处理 Result 的四种常见方式：

| 方式 | 适用场景 |
|------|---------|
| `match` | 需要分别处理 Ok 和 Err |
| `unwrap()` | 快速原型，失败就 panic |
| `expect("msg")` | 同上但带错误信息 |
| `?` 运算符 | 传播错误给调用者 |
| `unwrap_or(default)` | 提供默认值 |

## 解析用户输入的模式

```rust
let mut input = String::new();
std::io::stdin().read_line(&mut input)?;
let num: i32 = input.trim().parse()?;
```

这里两次用了 `?`：第一次是 IO 错误，第二次是解析错误。`parse()` 的返回类型由 `num` 的类型注解推导——这叫做**类型驱动的解析**，是 Rust 的特色。

## 触类旁通：Option vs Result

`Option<T>` 表达"可能有值"，`Result<T, E>` 表达"可能出错"：

- 找不到东西 → `Option`
- 操作失败 → `Result`
- `Option` 可以和 `?` 配合使用（在返回 `Option` 的函数中）
- `Result` 和 `Option` 可以互相转换：`.ok()` / `.ok_or()` / `.transpose()`

## 为什么 Rust 不用异常

1. **显式性**：函数签名不隐藏可能的失败路径
2. **零成本抽象**：Result 就是枚举，没有运行时开销
3. **强制处理**：`#[must_use]` 属性让编译器警告未处理的 Result
4. **组合性**：`map`、`and_then`、`or_else` 等方法链式处理，流水线风格

实际项目中，通常用 `anyhow`（应用层）或 `thiserror`（库层）简化错误处理。
