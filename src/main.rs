use clap::{App, Arg};

fn main() {
    let matches = App::new("rpwgen")
    .version("0.0.1")
    .author("Kotaro Yamashita")
    .about("Generate Passwords")
    .arg(
        Arg::with_name("length_of_password")
        .value_name("LENGTH_OF_PASSWORD")
        .short('l')
        .long("length")
        .index(1)
        .required(false),
    )
    .get_matches();

    match matches.value_of("length_of_password") {
        Some(length) => println!("length_of_password specified {}.", length),
        None => println!("length_of_password is not specified.")
    }
}