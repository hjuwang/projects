
use std::io;

fn main(){
    
    // 定义一个数组
    let a = [1,2,3,4,5];

    let mut index = String::new();

    //标准输入
    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    // 转换数据类型(变量覆盖)

    let index :usize= index.trim().parse().expect("index input was not a number");

    let emelemt = a[index];

    println!("the array in index {index} is {emelemt}");


    

}