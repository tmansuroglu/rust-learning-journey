// Scalar types are integer, floating point number, boolean and character.
fn main() {
    let a = 3;
    let b = -4;
    let x = 2.0;

    let y: f32 = 3.0;

    let z: char = 'A';
    let qwe = 'u';

    println!(
        " not truncated{} \n not truncated {}\n not truncated {}\n truncated {}",
        67.5 / 23.1,
        123.0 / 1.1,
        145.3 / 12.1,
        5 / 3
    );

    // Compound types are tuple, array and vector(?).

    // tuples have fixed size can't grow or shrink. each element can have a different type
    let tup1 = (x, y, z, a, b, qwe);
    println!("{}", tup1.0);

    let tup2: (i32, i32) = (a, b);

    let (a2, b2) = tup2;

    // The tuple without any values has a special name, unit.
    // This value and its corresponding type are both written () and represent an empty value or an empty return type.
    // Expressions implicitly return the unit value if they don’t return any other value

    // all array elements must be the same type
    // arrays have fixed length
    // allocated on stack
    // if you try to access an unexisting index the app will panic
    let a = [1, 2, 3, 4, 5, 6, 7];

    println!("{}", a[0]);
}

// statements are instructions that perform some action and do not return value. they have ; at the end
// expressions evaluate to a value. no ; at the end
