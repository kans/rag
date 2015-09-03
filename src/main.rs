use std::env;

pub mod output;
pub mod search;
pub mod utils;
extern crate ansi_term;
// static COLOR_LINE_NUMBER : String = "\033[1;33m"; /* bold yellow */
// use ansi_term::Colour::{Black, Red, Green, Yellow, Blue, Purple, Cyan, Fixed};
// use ansi_term::Style;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    println!("I require 2 arguments");
    std::process::exit(1);
  }

  let query = args[1].clone();
  let path : String = args[2].clone();
  let path = std::path::Path::new(&path);
  let s = search::Search::new(&query);
  s.search(path, false);
}