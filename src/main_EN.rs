
use std::io;
use rand::Rng;//api
use std::cmp ::Ordering;
//The referenced library, std is the standard library.

fn main() {
    println!("Guessing game！");  //It's a hint
    let secret_number = rand::thread_rng().gen_range(1..114);
    
    loop { //Loup
        println!("Please give your uess the number.");
        let mut guess = String ::new();  //声Declare the guess variable, string is a string, and add new to create a new variable.

        io ::stdin().read_line(&mut guess).expect("WARNING ERROR 1");  //Refer to the io library, stdin is the function, and the readline and expect methods are used.   
    
        
        println!("The number you gave me is：{}",guess);  //As a hint, {} is automatically replaced with the variable that follows.

        let guess: u32 =  match guess.trim().parse() {
           Ok(num) => num,
           Err(_)=> continue
        };//Determine whether the content entered by the user is a comparison of numbers
    
        match guess.cmp(&secret_number) { //The following is an arm that belongs to Ordering, and if it matches a branch, it will execute the code behind the pipeline.
        Ordering::Less => println!("No,to small!"),
        Ordering::Greater => println!("No,to big!"), 
        Ordering::Equal => {
            println!("You won！");
            println!("Bey");
            break;
        }
       }
    }

    
    

   

}
