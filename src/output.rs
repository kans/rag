use std;
use std::io;
use std::io::prelude::*;

use ansi_term::Colour::{Fixed};

static NEWLINE : u8 = 10;

pub fn print_matches(text: &Vec<u8>, position: isize, pattern_length: isize) {
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

pub fn stderr(err: std::io::Error) {
  writeln!(&mut io::stderr(), "ERR: {}", err.to_string()).unwrap();
}

pub fn stderr2(err: std::io::Error, message: &str) {
  writeln!(&mut io::stderr(), "ERR: {} - {}", message, err.to_string()).unwrap();
}