//Tell rust to "satisfy" Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//"defining" methods and "associated" functions
impl Rectangle {
    // "methods"
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // "associated" functions -> static methods
    fn square(value: u32) -> Rectangle {
        Rectangle {
            width: value,
            height: value,
        }
    }
}
// structure can have multiple impl clauses
impl Rectangle {
    fn increase_width(&mut self, value: u32) {
        self.width += value;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 15,
    };
    println!("Rectangle is {:?}", rect1); //displays in one line struct fields
    println!("Rectangle is {:#?}", rect1); //formats the display
    println!("Rectangle has area {}", rect1.area());

    let mut rect2 = Rectangle { ..rect1 };
    rect2.increase_width(20);
    println!("New Rectangle area is {}", rect2.area());

    println!("Size of square is {}", Rectangle::square(32).area());
}
