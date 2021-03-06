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
  query_length: isize,
  query_bytes: &'a[u8],
}

impl <'a> Search <'a> {
  pub fn new (query: &'a String) -> Search {

    let mut occ: Vec<isize> = Vec::with_capacity(ALPHABETSIZE as usize);

    for _ in 0..ALPHABETSIZE {
      occ.push(-1);
    }

    let bytes = query.as_bytes();

    for (i, a) in bytes.iter().enumerate() {
      occ[*a as usize] = i as isize;
    }

    Search {
      query: query,
      query_bytes: &bytes,
      query_length: bytes.len() as isize,
      occ: occ
    }
  }

  pub fn search (&self, path: &Path, print_file: bool) {
    let metadata = match std::fs::metadata(path) {
      Err(oh_shit) => {
        output::stderr(&oh_shit.to_string());
        process::exit(0);
      },
      Ok(metadata) => metadata,
    };

    if metadata.is_file() {
      self.search_file(path, print_file);
      return;
    }

    if metadata.is_dir() {
      self.search_dir(path);
    }
  }

  fn read_file(&self, path: &String, buf: &mut Vec<u8>) -> io::Result<()> {
    let mut f = try!(File::open(path));
    try!(f.read_to_end(buf));
    Ok(())
  }

  fn search_file (&self, path: &std::path::Path, should_print_file: bool) {
    let mut buf: Vec<u8> = Vec::new();
    let string : String = match path.to_str() {
      None => process::exit(0),
      Some(s) => format!("{}", s),
    };

    let result = self.read_file(&string, &mut buf);
    if let Err(e) = result {
      let message = format!("Can not read path {:?} - {}", &string, &e.to_string());
      output::stderr(&message);
      return;
    }

    let mut i: isize = 0;
    let mut j: isize;
    let mut printed_file = false;
    let pattern = self.query_bytes;
    let buf_len = buf.len();
    if utils::is_binary(&buf, buf_len) {
      return;
    }
    // TODO: make unsafe!
    while i < buf_len as isize - self.query_length {
      j = self.query_length - 1;
      while j >= 0 && pattern[j as usize] == buf[(i+j) as usize] {
        j -= 1;
      }
      if j < 0 {
        if should_print_file && !printed_file {
          printed_file = true;
          let p = format!("{:?}", path.display());
          println!("\n{}", Green.paint(&p));
        }
        output::print_matches(&buf, i, self.query_length, self.query);
      }
      i += self.query_length;
      i -= self.occ[buf[i as usize] as usize];
    }
  }

 fn search_dir (&self, path: &Path) {

    let entries = match fs::read_dir(path) {
      Err(oh_shit) => {
        output::stderr(&oh_shit.to_string());
        return;
      },
      Ok(entries) => entries
    };

    for entry in entries {
      let direntry = match entry {
        Err(oh_shit) => {
          output::stderr(&oh_shit.to_string());
          continue;
        },
        Ok(entry) => entry,
      };

      let path_buf : PathBuf = direntry.path();
      let fucking_path : &Path = path_buf.as_path();
      let metadata = match std::fs::metadata(fucking_path) {
        Err(oh_shit) => {
          output::stderr(&oh_shit.to_string());
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