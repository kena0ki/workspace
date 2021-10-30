
use std::io::prelude::*;
use std::io;

mod macros;
mod fncs;

use fncs::fn001;
use fncs::fn002;
use fncs::fn003;
use fncs::fn004;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    dispatch_to!(args[1].as_str(), args
        => {
            fn001,
            fn002,
            fn003,
            fn004,
        }
    ).unwrap();
}

fn _stdin_parse() -> Result<(), Box<dyn std::error::Error>> {
  let stdin = &mut io::stdin();
  let vec = &mut Vec::with_capacity(5*1024);
  stdin.read_to_end(vec)?;
  return Ok(());
}

