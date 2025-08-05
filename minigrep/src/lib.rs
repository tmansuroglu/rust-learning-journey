use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
pub struct Args<'a> {
    pub search_string: &'a String,
    pub file_dir: &'a String,
}

pub fn get_formatted_args(args: &Vec<String>) -> Result<Args, &'static str> {
    if args.len() != 3 {
        return Err("Usage: cargo run -- searchString fileDir.txt");
    }

    let search_string = &args[1];
    let file_dir = &args[2];

    Ok(Args {
        search_string,
        file_dir,
    })
}

pub fn get_file_in_str(file_dir: &String) -> String {
    let file = read_to_string(file_dir).expect("Unable to read file");

    file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_arg() {
        assert_eq!(
            get_formatted_args(&vec![String::from("hello")]),
            Err("Usage: cargo run -- searchString fileDir.txt")
        );
    }

    #[test]
    fn two_args() {
        assert_eq!(
            get_formatted_args(&vec![String::from("hello"), String::from("hello"),]),
            Err("Usage: cargo run -- searchString fileDir.txt")
        );
    }

    #[test]
    fn three_args() {
        assert_eq!(
            get_formatted_args(&vec![
                String::from("hello"),
                String::from("hello"),
                String::from("text.txt")
            ]),
            Ok(Args {
                search_string: &String::from("hello"),
                file_dir: &String::from("text.txt"),
            })
        );
    }

    #[test]
    fn four_args() {
        assert_eq!(
            get_formatted_args(&vec![
                String::from("hello"),
                String::from("hello"),
                String::from("hello"),
                String::from("hello"),
            ]),
            Err("Usage: cargo run -- searchString fileDir.txt")
        );
    }

    #[should_panic(expected = "Unable to read file")]
    #[test]
    fn wrong_file_dir() {
        get_file_in_str(&String::from("qwe"));
    }

    #[test]
    fn correct_file_dir() {
        assert!(
            !get_file_in_str(&String::from("text.txt")).is_empty(),
            "Should not be empty"
        );
    }
}
