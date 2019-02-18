fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area(w: i32, h: i32) -> i32 {
    w * h
}
