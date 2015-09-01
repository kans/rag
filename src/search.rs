use std;
use std::process;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use utils;
use output;
// use self::output;
// use self::utils;
use ansi_term::Colour::{Fixed, Green};
static ALPHABETSIZE : i32 = 256;

pub fn horspool_init_occ(pattern: &String) -> Vec<isize> {
  let mut vec: Vec<isize> = Vec::with_capacity(256);
  for _ in 0..(ALPHABETSIZE) {
    vec.push(-1);
  }

  for (i, a) in pattern.as_bytes().iter().enumerate() {
    vec[*a as usize] = i as isize;
  }
  // println!("{:?}", vec);
  vec
}


fn read_file(path: &String, buf: &mut Vec<u8>) -> io::Result<()> {
  let mut f = try!(File::open(path));
  try!(f.read_to_end(buf));
  Ok(())
}

pub fn handle_path (path: &std::path::Path, query: &String, should_print_file: bool) {
  let mut buf: Vec<u8> = Vec::new();
  let string : String = match path.to_str()  {
    None => process::exit(0),
    Some(s) => format!("{}", s),
  };

  read_file(&string, &mut buf).ok().expect("could not read the file");
  let occ = horspool_init_occ(&query);
  let mut i: isize = 0;
  let mut j: isize;
  let mut printed_file = false;
  let pattern = query.as_bytes();
  let pattern_length = pattern.len() as isize;
  let buf_len = buf.len();
  if utils::is_binary(&buf, buf_len) {
    return;
  }
  // let newline_cache: Vec<usize> = Vec::new();
  while i < buf_len as isize - pattern_length {
    j = pattern_length - 1;
    while j >= 0 && pattern[j as usize] == buf[(i+j) as usize] {
      j -= 1;
    }
    if j < 0 {
      if should_print_file && !printed_file {
        printed_file = true;
        let p = format!("{:?}", path.display());
        println!("{}", Green.paint(&p));
      }
      output::output(&buf, i, pattern_length);
    }
    i += pattern_length;
    i -= occ[buf[i as usize] as usize];
  }
}