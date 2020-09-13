fn main() {
    println!("Hello, world!");
    another_function(3, 5);
}

// functions
fn another_function (x: i32, y: i32) { // snake case is naming convention
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let y = { // a new scope is an expression
        let x = 3;
        x + 1 // omit semicolon to return expression
    };
    let fiver = five();
    let sixer = plus_one(five());
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
