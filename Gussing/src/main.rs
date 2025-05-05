extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing!");
   
    let mut flag_exit= String::new(); 

    loop{
        let  secret_number = rand::thread_rng().gen_range(1, 101);
        println!("The secret number is: {}", secret_number);
   
   loop{
    println!("enter the the number!");
    let mut guess =String::new();
    io::stdin().read_line(&mut guess).expect("failure to read data");
    let guess: u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue
    };
    
    println!("The user guess is {}", guess);

    match  guess.cmp(&secret_number){
        Ordering::Less=>println!("Too small"),
        Ordering::Greater=>println!("Too big"),
        Ordering::Equal=>{
            println!("Conguatulation you won!!");
            break;
                       
        },



    };
   }
   println!("do you want to continue Y|N?");
   io::stdin().read_line(&mut flag_exit).expect("failure to read data");
   let flag_exit = flag_exit.trim();
   println!("flag {} ",flag_exit);

   if flag_exit=="Y"||flag_exit=="y" {
        continue;
   }else {
       println!("Exit...");
       break;
   }

} 
}
