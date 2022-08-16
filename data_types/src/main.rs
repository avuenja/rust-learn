fn main() {
    //Floating-Point Types
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("x: {x}, y: {y}");

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("sum: {sum}, difference: {difference}, product: {product}, quotient: {quotient}, floored: {floored}, remainder: {remainder}");

    // Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("true: {t}, false: {f}");

    // Character Type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c: {c}, z: {z}, cat: {heart_eyed_cat}");

    // Compound Types
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (tup_x, tup_y, tup_z) = tup;

    println!("The value of tup_y is: {tup_y}, tup_x: {tup_x} and tup_z: {tup_z}");

    let five_hundred = tup.0;

    println!("Five hundred: {five_hundred}");

    // Array Type
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = a[0];

    println!("a: {first}");
}
