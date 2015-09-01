use std::io::prelude::*;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub mod output;
pub mod search;
pub mod utils;
extern crate ansi_term;
// static COLOR_LINE_NUMBER : String = "\033[1;33m"; /* bold yellow */
// use ansi_term::Colour::{Black, Red, Green, Yellow, Blue, Purple, Cyan, Fixed};
// use ansi_term::Style;

fn read_dir (path: &Path, query: &String, print_file: bool) {
  let metadata = std::fs::metadata(path).unwrap();
  if metadata.is_file() {
    search::handle_path(path, query, print_file);
    return;
  }
  if !metadata.is_dir() {
    return;
  }

  for entry in fs::read_dir(path).unwrap() {
    let direntry = entry.unwrap();

    let path_buf : PathBuf = direntry.path();
    let fucking_path : &Path = path_buf.as_path();
    let metadata = std::fs::metadata(fucking_path).unwrap();

    if metadata.is_file() {
      search::handle_path(fucking_path, query, true);
      continue;
    }
    if !metadata.is_dir() {
      continue;
    }
    read_dir(fucking_path, query, true);
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    println!("I require 2 arguments");
    std::process::exit(1);
  }

  let query = args[1].clone();
  let path : String = args[2].clone();
  let path = std::path::Path::new(&path);
  read_dir(path, &query, false);
}