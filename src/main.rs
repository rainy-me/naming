extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod api;
mod core;

fn main() {
    let info = core::get_naming_count();
    println!("{:#?}", info)
}
