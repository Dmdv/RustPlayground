#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

fn main() {

    let rect = Rect {width: 50, height: 30};

    let sq = area(&rect);

    println!("Area = {}", sq);
}

fn area(rect: &Rect) -> u32 {
    rect.height * rect.width
}
