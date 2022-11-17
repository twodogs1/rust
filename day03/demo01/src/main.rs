fn main() {
    fun_demo01();

    fun_demo02(1,1);

    let r1=fun_demo03(1,2);
    println!("r1: {}", r1);

    let r2=fun_demo04(2,3);
    println!("r2: {}", r2);

    //例1
    //let r3=(let aa = 1)

    //例2
    let r4={
        let a = 1;
        let b = 2;
        a+b
    };
    println!("r4: {}", r4);
}
//无参函数
fn fun_demo01(){
    println!("Hello world!");
}
//有参无返回值函数
fn fun_demo02(a:i32,b:i32){
    let result=a+b;
    println!("result: {}",result);
}
//有参有返回值-方式1
fn fun_demo03(a:i32,b:i32)->i32{
    let result=a+b;
    return result;
}
//有参有返回值-方式2
fn fun_demo04(a:i32,b:i32)->i32{
    let result=a+b;
    result
}

