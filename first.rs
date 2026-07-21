fn main(){
    println!("Heyyy");

    const LINK_TO_USE: &str = "LINK ME UP";
    println!("{}", LINK_TO_USE);

    // floating types
    {
        let x  = 2.0;
        let y: f32 = 3.0;
        println!("Value_X = {} and the value of y is {}", x, y);
    
        //math ops

        let sum  = x + y;

        let difference = y - x;

        let product = x * y;

        let quotient  = y/x;
        let truncated = -y / x;

        let remainder = y % x;

        println!("The operations on x and y: \n sum = {} \n diffference = {} \n product = {} \n quotient = {} \n truncated = {} \n remainder = {} \n", sum, difference, product, quotient, truncated, remainder)

    }
}