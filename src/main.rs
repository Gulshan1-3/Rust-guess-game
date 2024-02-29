
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
   
   let secret_number = rand::thread_rng().gen_range(1..=100);

  
   loop {
   
    println!("Enter the number:");

    let mut guess:String = String ::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read line");
    let guess : u32 = match
        
    guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };

    println!("you guessed:{}",guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Equal => {
            println!("The secret number is:{}",secret_number);
            break;
        },
        Ordering::Greater => 
            println!("too big"),

    }
}
}
