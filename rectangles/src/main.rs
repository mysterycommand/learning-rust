fn main() {
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect)
    );
}

fn area(rect: (u32, u32)) -> u32 {
    let (w, h) = rect;
    w * h
}
