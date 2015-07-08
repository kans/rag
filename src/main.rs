use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::process;
use std::env;

static ALPHABETSIZE : i32 = 256;
static NEWLINE : u8 = 10;
// static COLOR_LINE_NUMBER : String = "\033[1;33m"; /* bold yellow */
// static COLOR_RESET : String = "\033[0m\033[K";

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
  println!("{:.*}:{:?}", 2, number_of_newlines, s);
  // fprintf(out_fd, "%s%lu%s%c", opts.color_line_number, (unsigned long)count, color_reset, sep);
  // println!("{}{:.*}{}:{:?}", COLOR_LINE_NUMBER, 2, number_of_newlines, COLOR_RESET, s);
  // process::exit(0);
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

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    println!("I require 2 arguments");
  }

  let query = args[1].clone();
  let path = args[2].clone();
  let mut buf: Vec<u8> = Vec::new();

  read_file(&path, &mut buf).ok().expect("could not read the file");
  let occ = horspool_init_occ(&query);
  horspool_search(&query, &buf, &occ);
  // println!("{}", buf);
}