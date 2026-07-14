//use core::num;
use std::io;

fn main() {
    println!("Hello, world!");
    conv();
}

fn conv () -> (){

    println!("Celsius to fahrenheit converter");
    let mut chanc = 0;

    while chanc < 5 { // loop to keep the program running 5 times
    println!("Enter your celsius value");
    let mut fah: String = String::new();

    // take input from user
    io::stdin()
        .read_line(&mut fah)
        .expect("Couldnt read message");

    /*
    accepting and converting the string to float value   
    */
    let fah:f32 = match fah.trim().parse(){
        Ok(num)=> num,
        Err(_) => continue,
    };

    // carry out the conversion to fahrenheit
    let result = (fah - 32.0)*5.0/9.0;
     println!("The degrees in celsius are: {:?}", result);
          
    chanc +=1;
} // end loop
   
}

