use std::io;
use rand::Rng;//api
//引用的库，std是标准库，io应该是个类？

fn main() {
    println!("猜数小游戏hhh");  //就是个提示

    let secret_number = rand::thread_rng().gen_range(1..114);

    println!("妙妙数字就是：{}",secret_number);

    println!("你就先给个数，ok？");

    let mut guess = String ::new();  //声明guess变量，string是字符串，加个new是新建

    io ::stdin().read_line(&mut guess).expect("WARNING ERROR 1");  //引用io库，stdin为函数，使用readline和expect方法

    println!("给我的数：{}",guess);  //就是个提示{}会自动替换为后面的所写的变量





}
