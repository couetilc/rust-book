use std::io;

fn main() {
    let number: u32 = loop {
        let mut nth = String::new();
        println!("How many iterations of the Fibonacci sequence do you want to calculate?");
        io::stdin().read_line(&mut nth).expect("Failed to read line!");
        match nth.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("\ninvalid integer!\n");
                continue;
            }
        };
    };
    println!("The {}th Fibonacci number is {}", number, fibonacci(number));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
