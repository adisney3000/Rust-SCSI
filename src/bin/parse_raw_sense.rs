//#![warn(missing_docs)]

use scsi::Sense;
use std::io;
use std::io::BufRead;

fn main() {
  let mut buffer: Vec <u8> = Vec::new();

  let stdin = io::stdin();
  let stdin_handle = stdin.lock();
  let mut iter = stdin_handle.lines();
  while let Some(Ok(line)) = iter.next() {
    for token in line.split_whitespace() {
      buffer.push(u8::from_str_radix(&token, 16).expect("Invalid hex value"));
    }
  }

  let sense: Sense = Sense::from_buf(&buffer).expect("Invalid sense data!");
  println!("== Debug Print ==");
  println!("{:#?}", sense);
  println!("");
  println!("== Sense Print ==");
  println!("{}", sense);
}
