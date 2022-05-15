use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("猜数开始!");
    // 生成随机数
    let num = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("猜一个数字");
        let mut guess = String::new();
        // 读取用户输入的数字，赋值给guess
        io::stdin().read_line(&mut guess).expect("无法读取");
        println!("你猜测的数字是：{}", guess);

        // shadow 覆盖上面定义的变量，更改变量类型，用于下面的比较
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个正确的数字类型");
                continue;
            }
        };

        match guess.cmp(&num) {
            Ordering::Less => println!("你太小了"),
            Ordering::Greater => println!("你太大了"),
            Ordering::Equal => {
                println!("猜对啦~");
                break;
            }
        }
    }
}
