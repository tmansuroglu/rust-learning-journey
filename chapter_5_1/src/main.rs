struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct UnitLikeStruct; // when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

struct Person {
    name: String,
    age: u8,
    height: u8,
    weight: f32,
}

fn main() {
    let random = Person {
        name: String::from(""),
        age: 0,
        height: 190,
        weight: 136.5,
    };

    let mut tako = Person {
        name: String::from("Tarkan"), // if you remove this there will be error
        // Because ownership of random's name is moved here.
        age: 31,
        ..random
    };

    println!("Tarkan height is {}", tako.height);

    tako.height = 189;

    println!("Tarkan height is {}", tako.height);
    println!("Tarkan name is {}", tako.name);
    println!("Tarkan age is {}", tako.age);
    println!("Tarkan weight is {}", tako.weight);

    println!("Random height is {}", random.height);
    println!("Random name is {}", random.name);
    println!("Random age is {}", random.age);
    println!("Random weight is {}", random.weight);
}
