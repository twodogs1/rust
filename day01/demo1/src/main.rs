const MAX_POINTS:u32 = 10000;
fn main() {
    //1.变量的定义
    //定义变量使用let，如果要定义可变变量需要使用mut
    let a = 1;
    println!("a={}", a);

    let mut b:u32 = 1;
    println!("b={}", b);

    b=2;
    println!("b={}", b);

    //2.隐藏
    let mut b:f32 = 1.1;
    println!("b={}", b);

    //3.常量
    println!("MAX_POINTS={}", MAX_POINTS);
}
