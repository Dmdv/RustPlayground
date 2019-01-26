struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {

    let rect = Rect {width: 50, height: 30};

    println!("Area = {}", rect.area());
}
