fn main() {
    // assign number the value of the "function" inside the brackets
    let number = {
        let x = 3;
        // no semi colon to indicate a return
        x + 1
    };

    println!("{}", number);

    let result = add_numbers(4, 6);
    println!("{}", result);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    // the below all work in returning a value
    x + y
    
    // return x + y;

    /*
        let result = x + y;
        result
    */
}