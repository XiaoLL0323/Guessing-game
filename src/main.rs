
use std::io;
use rand::Rng;//api
use std::cmp ::Ordering;
//引用的库，std是标准库，io应该是个 “方法”

fn main() {

    println!("猜数小游戏hhh");  //就是个提示

    let secret_number = rand::thread_rng().gen_range(1..114); //声明一个随机数变量，范围是1到114
    
    loop { //loop循环

        println!("你就给个数，ok？");

        let mut guess = String ::new();  //声明guess变量，string是字符串，加个new是新建

        io ::stdin().read_line(&mut guess).expect("WARNING ERROR 1");  //引用io库，stdin为函数，使用readline和expect方法    
        
        println!("给我的数：{}",guess);  //就是个提示，{}为占位符，会自动替换为后面的所写的变量

        let guess: u32 =  match guess.trim().parse() { //将用户输入的内容转换为u32数据类型，以便和随机数变量做比较
           Ok(num) => num,
           Err(_)=> continue
        };//解决用户输入的不是数字的情况下，是和不是的比较
    
        match guess.cmp(&secret_number) { //以下是属于Ordering的arm（分支），如果匹配到某个分支就会执行管道符后面的代码
        Ordering::Less => println!("nonononono,to small!"),
        Ordering::Greater => println!("nonononono,to big!"), 
        Ordering::Equal => {
            println!("nb啊");
            println!("好啦，你可以走啦！");
            break;
        } //在“loop”中的代码会一直循环下去。在这里，直到猜中的数字等于随机数，才会执行“break”跳出循环
       }
    }

    
    

   

}
