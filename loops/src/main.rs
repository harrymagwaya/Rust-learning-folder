fn main() {
    println!("Hello, world!");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter +2;
            
        }   
    };
    println!("The result of the loop is {result}");

    let mut count = 0;
    let mut recount = 0;
    'upper: loop {
    'count_up: loop {
        println!("count: {count}");
        println!("Renew: {recount}"); 
        let mut remaining = 10;

        loop {
            println!("Remaining: {remaining}");
            if remaining == 9{
                break;
            }
            if count == 4{
                break 'count_up;
            }
            remaining -= 1;
            count += 1;
        }
        
    }
    
    if recount == 4{
        println!("Ended {recount}");
        break 'upper;
    }
    recount += 1;
}
    println!("last count {count}");


    let array = [10,20,30,40,50,60];
    let mut index = 0;
    while index < array.len() {
        println!("The value is {}", array[index]);
        index += 1;
    }

    for number in (1..10).rev(){
        println!("The number is: {}", number);
    }
}
