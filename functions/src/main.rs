fn main() {
    let x = five();
    let y = four();

    println!("x = {x},y = {y}");
}

fn another_function(x: i32) {
    println!("x is {x}")
}

// 带有返回值的函数

fn five() -> i32 {
    //单个一个数字也是表达式，表达式不需要分号
    5
}

fn four() -> i32 {
    // return 是语句，语句需要加分号
    return 4;
}
