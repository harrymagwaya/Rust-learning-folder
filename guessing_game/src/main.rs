use std::io;

fn main() {
    println!("Guess the number! ");
    println!("Please input your guess. ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line"); // to handle any failure

    
    println!("You guessed: {guess}");
   // println!("Hello, world!");

   let mut num = String::new();
    io::stdin()
         .read_line(& mut num)
         .expect("failed to read line");
         
    println!("added {num}");
    println!("{{num2 = 30}}");
}
