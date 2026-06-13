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
use rand::{Rng, thread_rng};

fn main() {
    let subject = vec![
        "晚风", "老猫", "孤灯", "浮云", "山雀", "墨客", "残月", "顽童",
    ];
    let adverb = vec![
        "月下", "窗边", "荒郊", "渡口", "柳边", "雨夜", "山前", "花间",
    ];
    let verb = vec![
        "轻吻", "偷走", "打翻", "闲看", "揉碎", "揽住", "踏破", "细数",
    ];
    let obj = vec![
        "流年", "星河", "旧梦", "炊烟", "尘缘", "霜华", "晚霞", "清愁",
    ];

    let si = thread_rng().gen_range(0..subject.len());
    let s = subject[si];
    let ai = thread_rng().gen_range(0..adverb.len());
    let a = adverb[ai];
    let vi = thread_rng().gen_range(0..verb.len());
    let v = verb[vi];
    let oi = thread_rng().gen_range(0..obj.len());
    let o = obj[oi];

    let new_word = format!("{}{}{}{}", s, a, v, o);
    println!("{}", new_word);
}
