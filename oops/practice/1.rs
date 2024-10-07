fn fibonacci(n: usize) {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        print!("{} ", a);
        let temp = a;
        a = b;
        b = temp + b;
    }
    println!(); // 换行
}

fn main() {
    let n = 10; // 你想打印的斐波那契数的数量
    fibonacci(n);
}
