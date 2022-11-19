//控制流语句
fn main() {
    //if语句
    let y=1;
    if (y == 1) {
        println!("y={}", y);
    }
    //if-else
    if y==1 {
        println!("y=1");
    }else{
        println!("y!=1");
    }
    //if-else if-else
    if y==1 {
        println!("y={}", y);
    }else if y==2 {
        println!("y={}", y);
    }else{
        println!("y={}", y);
    }
    //if语句表达式
    let x=if y==1 {
        5
    }else{
        6
    };
    //loop语句
    let mut cnt=1;
    loop{
        println!("inloop cnt={}", cnt);
        cnt +=1;
        if cnt == 10 {
            break;
        }
    }
    //表达式赋值loop
    let x=loop{
        if cnt == 20 {
            break cnt*2
        }
        cnt +=1;
    };
    //while语句
    while cnt<=25{
        println!("inwhile cnt={}", cnt);
        cnt+=1;
    }
    //for语句
    let arr:[u32;5]=[1,2,3,4,5];
    for elem in &arr{
        println!("elem={}", elem);
    }
}
