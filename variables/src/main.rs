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
}
