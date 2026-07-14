use std::ascii::AsciiExt;
use std::ops::{Range, RangeInclusive, Add, Shl};


fn main() {
    let  x = 8; // first delcaration of x
    println!("The number is {x}");


    let x = 6; // this x reassignment automatically over shadows the first declaration
    println!("The number is {x}");
    #[allow(dead_code)] // used to supress the warn(dead_code)
    const NEW_X: i32 = 19;
    println!("The new value of x is {x}");

    // using a tuple
    let tup: (u128, u8, f32) = (222, 19, 20.1);
    println!("{}",tup.2);

    let d = [3; 5];
    println!("{}",d[0]);

    // trying to test the overshadowing that is a main feature of variables in rust
    {
        let x = x * 5;
        println!("The insider value of x is {x}");
    }

    let  n = 2;
    // for i in 0..n {
    //     println!("_=> {} ",fibonacci(i));
    // }
    
    for j in 0..n+1{
        println!("_=> {}",fibo(j));
    }
    
}

fn fibo(n:usize)-> usize{
    if n < 0{
      panic!("{} is negative",n);
    }
    else if n == 1 {
        return 1;
    }
     let mut sum = 0;
     let mut prev = 0;
     let mut curr = 1;
        
     for _i in 1..n {
        sum = prev + curr;
        prev = curr;
        curr = sum;
     } 
    return  sum;
}

fn fibonacci(n: usize) -> usize{
    if n<=0 {
        return 0;
    }else if n==1 {
        return 1;
    }else {
        return fibonacci(n-1) + fibonacci(n-2);
    }

}

fn tuppl(){

    let tup (ch, int) = ('a', 2,3,4,5,6,7,89);
}


