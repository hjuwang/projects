fn main() {
    // let mut count = 0;

    // let result = loop {
    //     count += 1;

    //     if count == 10 {
    //         break count * 2;
    //     }
    // };

    // println!("result = {result}")

    let a = [1, 2, 3, 4, 5];
    // print_array(a);
    for_array(a);

    // for_demo();
}

fn print_array(a: [i32; 5]) {
    let mut index = 0;

    while index < 5 {
        println!("a[{}]={}", index, a[index]);
        index += 1;
    }
}

fn for_array(a: [i32; 5]) {
    for v in a {
        println!("{v}");
    }
}

fn for_demo() {
    for number in (1..4).rev() {
        println!("{number}");
    }
}
