struct Rectangle {
    width: u32,
    height: u32,
}

// all the functions inside this are called associated function
impl Rectangle {
    // also method because starts with &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // also method because starts with &self
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// you can have multiple impl
impl Rectangle {}

fn main() {
    let rct1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rct area: {}", rct1.area());

    let rct2 = Rectangle {
        width: 10,
        height: 50,
    };

    println!("rct2 area: {}", rct2.area());

    println!("rct1 can hold rct2: {}", rct1.can_hold(&rct2));

    println!("rct2 can hold rct1: {}", rct2.can_hold(&rct1));

    let sq = Rectangle::square(60);

    println!("sq can hold rct1 {}", sq.can_hold(&rct1))
}
