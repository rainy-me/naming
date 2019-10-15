extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate clap;
extern crate regex;
use clap::{App, Arg};

mod api;
mod core;

fn main() {
    let matches = App::new("naming")
        .version("v0.1.0")
        .arg(
            Arg::with_name("lang")
                .short("l")
                .long("lang")
                .use_delimiter(true)
                .takes_value(true)
                .help("Get Status of the given lang")
                .required(false),
        )
        .arg(
            Arg::with_name("repo")
                .required(true)
                .help("Github repo to scan"),
        )
        .get_matches();

    if let Some(repo) = matches.value_of("repo") {
        let mut info = core::get_naming_count(repo).unwrap();
        if let Some(maybe_langs) = matches.values_of("lang") {
            let langs = maybe_langs.collect::<Vec<&str>>();
            info.retain(|key, _| langs.contains(&key.as_str()));
        }

        println!(
            "{}",
            serde_json::to_string_pretty(&info).expect("Failed to serialize data")
        );
    }
}
