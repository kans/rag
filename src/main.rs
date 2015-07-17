use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::process;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

extern crate ansi_term;
use ansi_term::Colour::{Fixed, Green};
// static COLOR_LINE_NUMBER : String = "\033[1;33m"; /* bold yellow */
// use ansi_term::Colour::{Black, Red, Green, Yellow, Blue, Purple, Cyan, Fixed};
// use ansi_term::Style;

static ALPHABETSIZE : i32 = 256;
static NEWLINE : u8 = 10;

fn output(text: &Vec<u8>, position: isize, pattern_length: isize) {
  let mut start: isize = position;

  while start > 0 {
    if text[start as usize] == NEWLINE {
      start += 1;
      break;
    }
    start -= 1;
  }

  let mut end: isize = position + pattern_length;
  while end <= text.len() as isize {
    if text[end as usize] == NEWLINE {
      break;
    }
    end += 1;
  }
  if end - start > 200 {
    end = start + pattern_length as isize;
  }

  let mut _string: Vec<u8> = Vec::new();
  for a in start..end {
    _string.push(text[a as usize]);
  }
  let s = match std::str::from_utf8(&_string) {
    Ok(v) => v,
    Err(e) => panic!("Invalid utf8 {}", e),
  };

  // get number of previous newlines
  let mut index = position - 1;
  let mut number_of_newlines = 1;
  while index >= 0 {
    if text[index as usize] == NEWLINE {
      number_of_newlines += 1;
    }
    index -= 1;
  }
  let number_string : String = number_of_newlines.to_string();
  println!("{:.*}:{:?}", 3, Fixed(33).paint(&number_string), s);
}

fn read_file(path: &String, buf: &mut Vec<u8>) -> io::Result<()> {
  let mut f = try!(File::open(path));
  try!(f.read_to_end(buf));
  Ok(())
}

fn horspool_search (pattern: &String, text: &Vec<u8>, occ: &Vec<isize>) {
    let mut i: isize = 0;
    let mut j: isize;

    let pattern = pattern.as_bytes();
    let pattern_length = pattern.len() as isize;

    // let newline_cache: Vec<usize> = Vec::new();
    while i < text.len() as isize - pattern_length {
      j = pattern_length - 1;
      while j >= 0 && pattern[j as usize] == text[(i+j) as usize] {
        j -= 1;
      }
      if j < 0 {
        output(text, i, pattern_length);
      }
      i += pattern_length;
      i -= occ[text[i as usize] as usize];
  }
}

fn horspool_init_occ(pattern: &String) -> Vec<isize> {
  let mut vec: Vec<isize> = Vec::with_capacity(256);
  for _ in 0..(ALPHABETSIZE-1) {
    vec.push(-1);
  }

  for (i, a) in pattern.as_bytes().iter().enumerate() {
    vec[*a as usize] = i as isize;
  }
  // println!("{:?}", vec);
  vec
}

fn read_dir (path: &Path, query: &String, print_file: bool) {
  let metadata = std::fs::metadata(path).unwrap();
  if metadata.is_file() {
    handle_path(path, query, print_file);
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
      handle_path(fucking_path, query, true);
      continue;
    }
    if !metadata.is_dir() {
      continue;
    }
    read_dir(fucking_path, query, true);
  }
}

fn handle_path (path: &std::path::Path, query: &String, print_file: bool) {
  let mut buf: Vec<u8> = Vec::new();
  let string : String = match path.to_str()  {
    None => process::exit(0),
    Some(s) => format!("{}", s),
  };

  read_file(&string, &mut buf).ok().expect("could not read the file");
  let occ = horspool_init_occ(&query);
  if print_file {
    let p = format!("{:?}", path.display());
    println!("{}", Green.paint(&p));
  }
  horspool_search(&query, &buf, &occ);
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    println!("I require 2 arguments");
  }

  let query = args[1].clone();
  let path : String = args[2].clone();
  let path = std::path::Path::new(&path);
  read_dir(path, &query, false);
}