use std;
use std::process;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use utils;
use output;

use ansi_term::Colour::{Green};

static ALPHABETSIZE : i32 = 256;

pub struct Search<'a> {
  occ: Vec<isize>,
  query: &'a String,
}

impl <'a> Search <'a> {
  pub fn new (query: &'a String) -> Search <'a> {
    let mut occ: Vec<isize> = Vec::with_capacity(ALPHABETSIZE as usize);
    for _ in 0..(ALPHABETSIZE) {
      occ.push(-1);
    }

    for (i, a) in query.as_bytes().iter().enumerate() {
      occ[*a as usize] = i as isize;
    }

    Search {
      query: query,
      occ: occ
    }
  }

  pub fn search (&self, path: &Path, print_file: bool) {
    let metadata = std::fs::metadata(path).unwrap();

    if metadata.is_file() {
      self.search_file(path, print_file);
      return;
    }
    if !metadata.is_dir() {
      return;
    }

    self.search_dir(path);
  }

  fn read_file(&self, path: &String, buf: &mut Vec<u8>) -> io::Result<()> {
    let mut f = try!(File::open(path));
    try!(f.read_to_end(buf));
    Ok(())
  }

  fn search_file (&self, path: &std::path::Path, should_print_file: bool) {
    let mut buf: Vec<u8> = Vec::new();
    let string : String = match path.to_str()  {
      None => process::exit(0),
      Some(s) => format!("{}", s),
    };

    self.read_file(&string, &mut buf).ok().expect("could not read the file");
    let mut i: isize = 0;
    let mut j: isize;
    let mut printed_file = false;
    let pattern = self.query.as_bytes();
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
      i -= self.occ[buf[i as usize] as usize];
    }
  }

 fn search_dir (&self, path: &Path) {
    let entries = match fs::read_dir(path) {
      Err(oh_shit) => {
        writeln!(&mut io::stderr(), "{}", oh_shit).unwrap();
        return;
      },
      Ok(entries) => entries
    };

    for entry in entries {
      let direntry = match entry {
        Err(oh_shit) => {
          writeln!(&mut io::stderr(), "{}", oh_shit).unwrap();
          continue;
        },
        Ok(entry) => entry,
      };

      let path_buf : PathBuf = direntry.path();
      let fucking_path : &Path = path_buf.as_path();
      let metadata = match std::fs::metadata(fucking_path) {
        Err(oh_shit) => {
          writeln!(&mut io::stderr(), "{}", oh_shit).unwrap();
          continue;
        },
        Ok(metadata) => metadata,
      };

      if metadata.is_file() {
        self.search_file(fucking_path, true);
        continue;
      }
      if !metadata.is_dir() {
        continue;
      }
      self.search_dir(fucking_path);
    }
  }
}