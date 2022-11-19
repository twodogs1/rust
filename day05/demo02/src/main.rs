//函数作用域
fn main() {
    let aa=String::from("hello");
    let i:i32=1;
    takes_ownership(aa);
    //这里的aa是没法访问的，函数的赋值和之前的作用域赋值是一样的，相当于aa在传入函数结束，aa的生命就终止了
    //println!("aa={}", aa);
    let bb=String::from("world");
    let bb=takes_ownership2(bb);
    //如果想要可以访问需要函数返回出来
    println!("bb={}", bb);
    takes_basic(1);
    //对于基本数据类型是可以进行访问的同之前函数内的作用域一样
    println!("i={}", i);
}

fn takes_ownership(aa:String){
}

fn takes_ownership2(bb:String)->String{
    bb
}

fn takes_basic(i:i32){
}
