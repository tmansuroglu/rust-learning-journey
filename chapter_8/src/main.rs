use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut v = Vec::new();

    v.push(1);

    let mut v2 = vec!["a", "b", "c", "d", "e", "f", "g"];

    // if you code &v2[100] the app will panic
    let t = &v2[0];

    // if you code v.get(100) return value is None. So no panic
    let t2 = v.get(0);

    for i in &v2 {
        println!("{}", i)
    }

    for el in v2.iter() {
        println!("{}", el)
    }

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{}", s);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{}", scores.get(&String::from("Blue")).copied().unwrap());

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values, as demonstrated in Listing 8-22.
}
