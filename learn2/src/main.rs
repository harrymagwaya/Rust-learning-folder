use std::result;

fn main() {
    println!("Hello, world!");

    // functions 
   let y = {
        let x = 5;
        x+1
        
    };
    println!("{}", y);
    

    let cond  = true;
    let result_ = if cond{
        let v = 12;
        println!("{}",v);
    } else {println!("{}",9)};

    let result1 = _two(2);

    let num  = if cond {result_ } else {result_};

    println!("{}", result1 );
    
}


 fn _two (z: i32) -> i32{
        let _z = {
          let z = z * 32;
        

            println!("{}", z)
        };
        z
        
    }


