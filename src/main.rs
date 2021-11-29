use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜猜1-10数字");

    let secret_number = rand::thread_rng().gen_range(1..11);
    println!("秘密数字是：{}", secret_number);

    loop {
        println!("请输入你的数字");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("读取失败了");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入1-10范围内的数字");
                continue;
            },
        };

        println!("你猜的是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("猜对了，算你厉害!");
                break
            }
        }
    }
}
