
fn main() {
    let array1 = [255_u8; 5] ;
    println!("{:?}", array1);
    let array2: [u32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array2);
    let array3 = ["a", "b", "c"];
    println!("{:?}", array3);
    let array4 = ['a', 'b', 'c'];
    println!("{:?}", array4);
    // array4[0] = 'A';  // 报错不可变数组
    let mut array5 = [false, true];
    array5[0] = true;
    println!("{:?}", array5);

    let my_array=[10,-67,88,-5,-2,99,132,42];

    let mut result = 0;
    for num in &my_array {
        result += num;
    }
    println!("相加总和等于：{}", result);
    println!("{:?}", my_array);
}