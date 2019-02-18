#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };

    println!("rect is {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
