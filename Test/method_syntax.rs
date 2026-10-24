// methods are defined within the context of a struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// methods can take multiple params that we add to the signature after the self params
//
// Associated functions that aren’t methods are often used for constructors
// that will return a new instance of the struct.
impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 60,
    };

    let rect2 = Rectangle {
        width: 90,
        height: 90,
    };

    let sq = Rectangle::square(5);
    println!("{:#?}", sq);

    println!(
        "The area of Rectangle {:?} is {}",
        rect1,
        rect1.calculate_area()
    );

    println!("Can rect 1 hold rect 2? {}", rect2.can_hold(&rect1));
}
