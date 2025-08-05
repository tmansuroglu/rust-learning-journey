use minigrep::{get_file_in_str, get_formatted_args};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let formatted_args = get_formatted_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    let file_in_str = get_file_in_str(formatted_args.file_dir);

    let does_file_contain_search_string = file_in_str.contains(formatted_args.search_string);

    println!("{}", does_file_contain_search_string)
}
