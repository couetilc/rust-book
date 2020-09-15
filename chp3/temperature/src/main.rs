use std::io;

fn main() {
    let temperature = loop {
        let mut temperature = String::new();
        println!("Input the temperature:");
        io::stdin().read_line(&mut temperature).expect("Failed to read line!");
        match temperature.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("\ninvalid number!\n");
                continue;
            }
        };
    };

    loop {
        let mut unit = String::new();
        println!("What temperature you converting to? Type c for Celsius or f for Fahrenheit:");
        io::stdin().read_line(&mut unit).expect("Failed to read line!");
        let unit = unit.trim();

        if unit == 'f'.to_string() {
            break println!("Result: {} °F", c2f(temperature));
        } else if unit == 'c'.to_string() {
            break println!("Result: {} °C", f2c(temperature));
        } else {
            println!("\ninvalid unit!\n");
            continue;
        }
    };
}

fn c2f(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn f2c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
