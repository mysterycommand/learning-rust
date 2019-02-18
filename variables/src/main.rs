fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let x = "five";
    let x = x.len();
    let x = x * 3 + 1;
    println!("The value of x is: {}", x);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("The value of x is: {}, and y is: {}", x, y);

    // addition
    let sum = 5 + 10;
    println!("The sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The quotient is: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("The remainder is: {}", remainder);

    let tup = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", tup.2);
}
