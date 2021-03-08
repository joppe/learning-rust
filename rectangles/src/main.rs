#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width() >= rect.width() && self.height() >= rect.height()
    }

    fn area(&self) -> i32 {
        self.width() * self.height()
    }

    fn width(&self) -> i32 {
        (self.bottom_right.x - self.top_left.x).abs()
    }
    
    fn height(&self) -> i32 {
        (self.bottom_right.y - self.top_left.y).abs()
    }
}

fn main() {
    let rect = Rectangle { top_left: Point::from(0, 0), bottom_right: Point::from(1024, 768) };
    let rect2 = Rectangle { top_left: Point::from(0, 0), bottom_right: Point::from(800, 6000) };

    println!("The area {}", rect.area());
    println!("Rectangle {:#?}", rect);
    println!("Rectangle {:#?}", rect2);
    println!("rect can hold rect2 {}", rect.can_hold(&rect2));
}
