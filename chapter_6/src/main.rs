struct Ipv4Addr(u8, u8, u8, u8);

struct Ipv6Addr(u8, u8, u8, u8);

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum IpAddrV2 {
    V4 { ip: String },
}

enum Shape {
    Circle(f64),         // radius
    Rectangle(f64, f64), // width, height
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    pub(crate) fn existed_in(&self, p0: i32) -> bool {
        todo!()
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    //  If it's Coin::Quarter(some_state), the inner value (state) is bound to the variable state, and execution continues.
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    println!("Hello, world!");

    let config_max = Some(3u8);
    // The code in the if let block only runs if the value matches the pattern. In this case, if it is 'Some'
    // assignment to a variable is not necessary
    let x = if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
        1
    } else {
        0
    };
}
