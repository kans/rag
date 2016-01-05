use std::env;

pub mod output;
pub mod search;
pub mod utils;
extern crate ansi_term;

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