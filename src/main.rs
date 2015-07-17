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


/* This function is very hot. It's called on every file. */
fn is_binary(buf: &Vec<u8>, buf_len: usize) -> bool {
  if buf_len == 0 {
    return true;
  }

  if buf_len >= 3 && buf[0] == 0xEF && buf[1] == 0xBB && buf[2] == 0xBF {
    /* UTF-8 BOM. This isn't binary. */
    return false;
  }

  // if (buf_len >= 4 && strncmp(buf, "%PDF-", 5) == 0) {
  //     /* PDF. This is binary. */
  //     return 1;
  // }
  let mut suspicious_bytes: usize = 0;
  let mut total_bytes: usize = buf.len();
  if total_bytes > 512 {
    total_bytes = 512;
  }
  let mut i: usize = 0;
  while i < total_bytes {
    if buf[i] == 0 {
      /* NULL char. It's binary */
      return true;
    }
    if (buf[i] < 7 || buf[i] > 14) && (buf[i] < 32 || buf[i] > 127) {
      /* UTF-8 detection */
      if buf[i] > 193 && buf[i] < 224 && i + 1 < total_bytes {
        i += 1;
        if buf[i] > 127 && buf[i] < 192 {
          continue;
        }
      } else if buf[i] > 223 && buf[i] < 240 && i + 2 < total_bytes {
        i += 1;
        if buf[i] > 127 && buf[i] < 192 && buf[i + 1] > 127 && buf[i + 1] < 192 {
          i += 1;
          continue;
        }
      }
      suspicious_bytes += 1;
      /* Disk IO is so slow that it's worthwhile to do this calculation after every suspicious byte. */
      /* This is true even on a 1.6Ghz Atom with an Intel 320 SSD. */
      /* Read at least 32 bytes before making a decision */
      if i >= 32 && (suspicious_bytes * 100) / total_bytes > 10 {
        return true;
      }
    }
    i += 1;
  }

  if (suspicious_bytes * 100) / total_bytes > 10 {
    return true;
  }

  return false;
}

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
    Err(_) => return,
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

fn horspool_init_occ(pattern: &String) -> Vec<isize> {
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

fn handle_path (path: &std::path::Path, query: &String, should_print_file: bool) {
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
  if is_binary(&buf, buf_len) {
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
      output(&buf, i, pattern_length);
    }
    i += pattern_length;
    i -= occ[buf[i as usize] as usize];
  }
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