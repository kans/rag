static PDF_HEADER: &'static [u8] = &[0x25,0x50,0x44,0x46,0x2d];

/* This function is very hot. It's called on every file. */
pub fn is_binary(buf: &Vec<u8>, buf_len: usize) -> bool {
  // TODO: make unsafe!
  if buf_len == 0 {
    return true;
  }

  if buf_len >= 3 && buf[0] == 0xEF && buf[1] == 0xBB && buf[2] == 0xBF {
    /* UTF-8 BOM. This isn't binary. */
    return false;
  }

  if buf_len > 4 && &buf[0..5] == PDF_HEADER {
    /* PDF. This is binary. */
    return true;
  }
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