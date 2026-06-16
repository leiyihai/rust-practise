fn main() {
    // 显示指定变量的几种方式
    let _var1: u32 = 123;
    let _var2 = 123_u32;
    let _var3 = 123f64;
    let _var3 = 123 as f32;

    // 字符ascii互转
    let ascii_a = b'a';
    println!("{}", ascii_a);
    let char_a = ascii_a as char;
    println!("{}", char_a);
}
