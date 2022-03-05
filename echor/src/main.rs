use clap::{Arg, Command};
fn main() {
    // println!("{:?}", std::env::args());
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Node JX <fgsoap@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    let text = matches.values_of("text").unwrap().collect::<Vec<&str>>();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
