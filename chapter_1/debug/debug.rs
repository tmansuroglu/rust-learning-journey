// this can't be printed with fmt::Display or fmt::Debug
struct UnPrintable(i32);

// this is printable
#[derive(Debug)]
struct Printable(i32);

#[derive(Debug)]
struct X(i32);
#[derive(Debug)]
struct Y(X);

struct Person<'a> {
    name: &'a str,
    age:u8
}

// ':?' means use debug implementation to format this value
fn main(){
    println!("{:?} printed here.",Printable(22));

    println!("{:?} printed here.", Y(X(2)));
}