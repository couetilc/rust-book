const MAX_POINTS: u32 = 100_000;

fn main() {
    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    // integer types
    let typeSignedInt: i32 = 12; // default (signed uses's two's complement)
    let typeUnsignedInt: u32 = 12;
    // floating-point types
    let typeSinglePoint: f32 = 3.2;
    let typeDoublePoint: f64 = 6.4; // default (just as fast as single-point)
    // Numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    // boolean type
    let t = true;
    let f: bool = false; // explicity type annotation
    // character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring using pattern matching or
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    // array type
    let a = [1, 2, 3, 4, 5]; // allocated on the stack
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // with type annotations
    let a = [3; 5]; // with array initialization = [3, 3, 3, 3, 3]
}
