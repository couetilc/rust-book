fn main() {
    let number = 3;
    // 2 arms
    if number < 7 {
        println!("condition was true");
    } else {
        println!("condition was true");
    }
    // 4 arms
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number  % 3 == 0 {
        println!("number is divisible by 3");
    } else if number  % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // if is an expression
    let condition = true;
    let val = if condition { 5 } else { 6 };
}
