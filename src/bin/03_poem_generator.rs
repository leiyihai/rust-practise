// 小诗生成器
// ============
// 任务：从几个数组中随机选取词语，拼接成一句搞怪的小诗。
//
// 提示：
// - 用数组或 Vec<&str> 存储不同词性的词（主语、谓语、宾语、状语等）
// - 需要 rand 依赖，使用 thread_rng().gen_range(0..arr.len()) 获取随机索引
// - 或者用 slice 的 .choose() 方法（需要引入 rand::seq::SliceRandom）
// - 用 format! 宏拼接字符串
// - 每次运行可以生成不同的句子
// - 思考：&str 和 String 的区别是什么？什么时候用哪个？
//
// 运行方式：cargo run --bin 03_poem_generator
