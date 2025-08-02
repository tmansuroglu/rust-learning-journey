fn main() {
    // format! write formatted text to String
    // print! same as format! but text is printed to console (io::stdout)
    // println! same as print! but new line is appended
    // eprint! same as print! but text is printed to the standard error (io::stderr)
    // eprintln! same as eprint! but a new line is appended

    println!("{} days", 30);

    println!(
        "{0} {1} {2}",
        "This is first", "This is second", "This is third"
    );

    println!(
        "{first}st {second}nd {third}rd",
        first = 1,
        second = 2,
        third = 3
    );

    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // only the types that implement fmt::Display can be formatted
}
