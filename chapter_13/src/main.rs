use std::thread;

// The closure captures an immutable reference to the 'self' instance and passes it with the code we specify to the target (method or something else)
// Functions, on the other hand, are not able to capture their environment in this way.
// Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do.
// Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario.
// Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs closure type annotations too).

fn main() {
    let example_closure = |x| x; // closure refers to type from the usage

    let s = example_closure(String::from("hello"));

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}"); // captures an immutable reference to the vector named list

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    fn mutable_reference_to_vector() {
        print!("---------------------------------");
        println!("mutable reference to vector");
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        let mut borrows_mutably = || list.push(7); // captures a mutable reference to the vector named list

        // println!("Before calling closure: {list:?}"); This won't work because no other borrows are allowed when there’s a mutable borrow. First, borrows_mutably needs to be done.
        borrows_mutably();
        println!("After calling closure: {list:?}");
    }

    mutable_reference_to_vector();

    let new_list = vec![1, 2, 3];
    println!("Before defining thread closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}")) // to move ownership to a closure.
        .join()
        .unwrap();

    // Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership.
    // A closure body can do any of the following:
    // - move a captured value out of the closure,
    // - mutate the captured value,
    // - neither move nor mutate the value,
    // - capture nothing from the environment to begin with.
    // The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use.
    // Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
    // - FnOnce applies to closures that can be called once. All closures implement at least this trait because all closures can be called.
    //   - A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
    // - FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
    // - Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment.
    //   - These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn x() {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        list.sort_by_key(|r| r.width); // The reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple times: once for each item in the slice. The closure |r| r.width doesn’t capture, mutate, or move out anything from its environment, so it meets the trait bound requirements.
        println!("{list:#?}");
    }

    fn y() {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        // let mut sort_operations = vec![];
        let value = String::from("closure called");

        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            // sort_operations.push(value); // When we try to compile this code, we get this error that value can’t be moved out of the closure because the closure must implement FnMut:
            r.width
        });
        println!("{list:#?}");
    }

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.
}

// How the iterator is trait implemented
// pub trait Iterator {
//     type Item;
//
//     fn next(&mut self) -> Option<Self::Item>;
//
//     // methods with default implementations elided
// }

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1)); // immutable references
    assert_eq!(v1_iter.next(), Some(&2));
}

// The Iterator trait has a number of different methods with default implementations provided by the standard library;
// Some of these methods call the next method in their definition, which is why you’re required to implement the next method when implementing the Iterator trait.
// Methods that call next are called 'consuming adapters' because calling them uses up the iterator.
// 'Iterator adapters' are methods defined on the Iterator trait that don’t consume the iterator.

// v1.iter().map(|x| x + 1); map method is an iterator adapter because creates a new iterator.
// But iterators are lazy and this code is doing nothing.
// To use it:
// let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect is a consuming adapter

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
