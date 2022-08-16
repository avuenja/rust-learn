fn main() {
    println!("Hello, world!");

    another_function();
    with_params(5);
    more_params(5, 'h');
    statement();

    let return_value = plus_one(five());
    println!("The value of five is: {return_value}");
}

fn another_function() {
    println!("Another function.");
}

fn with_params(x: i32) {
    println!("The value of x is: {x}");
}

fn more_params(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements and Expressions
fn statement() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
