// Stack and heap are parts of the memory.
// Both are available at the run time.
//

// STACK
// - Stores values in order.
// - Last in First out.
// - Pushing onto stack (adding to stack)
// - Removing from stack (popping off the stack)
// - All data stored on the stack must have known fixed size.
// - Faster than the heap allocation. Because no search for space.
// - Accessing to data is also faster. No pointer following.

// HEAP
// - Less organized
// - Fixed size. You request a certain amount of space to store data.
// - Allocator finds the empty spot in the heap big enough for your request and marks it as used.
// - Returns a pointer. The pointer can be stored in the stack.
// - This is called allocating the heap.

// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// string literal vs String
// - string literal content is known ahead of compile time. Immutable. Thus fast and efficient.
// - String is mutable
// - string literal's value is stored in static memory
// - string literal's pointer is stored in stack
// - String's value is stored in heap
// - String's pointer is stored in stack

// This won't work because Rust drops s1 once the value reference assigned to s2
// let s1 = String::from("hello");
// let s2 = s1;
//
// println!("{s1}, world!");

// This works because integers have known size and stored entirely on stack and easy to copy.
// No difference between deep and shallow copy here.
// Let x = 5;
// let y = x;
//
// println!("x = {x}, y = {y}");

// We also cannot have a mutable reference while we have an immutable one to the same value.
//
// Why data race is bad and you can't borrow twice in one scope.
// Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There’s no mechanism being used to synchronize access to the data.

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}", r1, r2); // r1 and r2 are not used after this line.

    let r3 = &mut s; // NO PROBLEM

    println!("{}", r3);
}

// The Rules of References
//
// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.
