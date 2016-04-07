extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("mosaic")
                    .version("0.0.1")
                    .author("Richard Sentino")
                    .about("Photo Mosaic Generator")
                    .arg(Arg::with_name("file")
                            .short("f")
                            .long("file")
                            .value_name("FILE")
                            .takes_value(true))
                    .arg(Arg::with_name("output")
                            .short("o")
                            .index(1))
                    .get_matches();

    if let Some(f) = matches.value_of("FILE") {
        println!("Value for file: {}", f);
    }

}
