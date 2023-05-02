use clap::{App, Arg};

fn main() {
   let matches =  App::new("echo_rs")
       .version("0.1.0")
       .author("Thomas Whitworth")
       .about("A simple echo program written in Rust")
       .arg(
          Arg::with_name("text")
              .value_name("TEXT")
              .help("Input text")
              .required(true)
              .min_values(1),
       )
       .arg(
          Arg::with_name("omit_newline")
              .short("n")
              .help("Do not output the trailing newline")
              .required(false)
              .takes_value(false),
       )
       .get_matches();

    let text = matches.values_of("text").unwrap().collect::<Vec<&str>>();
    let omit_newline = matches.is_present("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}
