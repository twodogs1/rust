fn main() {
    let a = 8;
    let b:u32 = 7;
    let c:i32 = -11;

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);

    println!("--------------------------------");
    let aa:char = 'a';
    let bb:char = '我';

    println!("aa: {}", aa);
    println!("bb: {}", bb);

    println!("--------------------------------");
    let arr:[u32;2]=[1,2];
    println!("arr[0]={}", arr[0]);
    test_array(arr);

    println!("--------------------------------");
    let tup:(u32,char) =(1,'a');
    println!("tup.0={}", tup.0);
    println!("tup.1={}", tup.1);

    let tup=(1,'a');
    println!("tup.0={}", tup.0);
    println!("tup.1={}", tup.1);

    println!("--------------------------------");
    let a_bool:bool= true;
    println!("a_bool: {}", a_bool);

    println!("--------------------------------");
    let a_f:f32= 1.111;
    println!("a_f: {}", a_f);
}

fn test_array(arr:[u32;2]) {
    println!("arr[0]={}",&arr[1]);
}
