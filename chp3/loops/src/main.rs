fn main() {
    let mut count = 0;
    loop {
        println!("again!");
        count += 1;
        if count == 2 {
            break;
        }
    }

    count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is {}", result);
    
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    // error prone, what if index >= 5? rust will panic.
    // slow, compiler adds runtime code for the conditional check on every iteration
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // instead
    for element in a.iter() {
        println!("the value is : {}", element);
    }
    // let's replace our countdown with a for loop
    for number in (1..4).rev() { // rev => reverse
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
    // TODO: convert temperatures between Fahrenheit and Celsius
    // TODO: generate the nth Fibonacci number
    // TODO: Print the lyrics to the Christmas carol "The Twelve Days of Christmas"
    // taking advantage of the repetition in the song.
    // TODO: compile to webassembly and host these small programs on my website
}
