use regex::{Regex};
use std::{fs};

use lazy_static::lazy_static;

static AERON_REGEX: &str = "aeron_file=.*";

fn main() {
  //let args: Vec<String> = env::args().collect();
}

fn get_aeron_file(arg: &str) -> Option<&str> {
  lazy_static! {
    static ref MODE: Regex = Regex::new(AERON_REGEX).unwrap();
  }
  // error: look-around, including look-ahead and look-behind, is not supported
  if MODE.is_match(arg) {
    let split: Vec<&str> = arg.split('=').collect();
    let last: Option<&&str> = split.last();
    if last.is_some() && !last.unwrap().is_empty() {
      return Some(last.unwrap());
    }
  }
  None
}

fn read_aeron_file(file: &str) -> usize {
  let data = fs::read(file).expect("Unable to read file");
  return data.len()
}

#[cfg(test)]
mod tests {
  use crate::{get_aeron_file, read_aeron_file};

  #[test]
  fn get_aeron_file_test() {
    assert_eq!(get_aeron_file("aeron_file=pom.dat"), Some("pom.dat"));
    assert_eq!(get_aeron_file("aeron_file="), None);
    assert_eq!(get_aeron_file("pom=pouet"), None);
  }

  #[test]
  fn read_aeron_file_test() {
    assert_eq!(read_aeron_file("resources/test/cnc.dat"), 8392704);
  }
}