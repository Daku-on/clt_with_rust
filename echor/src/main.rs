use clap::{App, Arg};

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Dakuon <takako.recruit@gmail.com>")
        .about("Rust echo")
        .get_matches();
}
