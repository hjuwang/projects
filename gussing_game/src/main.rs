use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secrect_number = rand::thread_rng().gen_range(1..=100);
    println!("guess the number");

    loop {
        println!("please input a guess");
        let mut guess = String::new();
        io::stdin() // :: 表示使用关联函数
            .read_line(&mut guess) // . 表示调用实例方法
            .expect("Failed to read line");

        //转换guess 的类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed {}", guess);

        match guess.cmp(&secrect_number) {
            // 枚举值
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
