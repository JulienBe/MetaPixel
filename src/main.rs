use regex::{Match, Regex};
use std::env;
use lazy_static::lazy_static;

static aeron_regex: &str = "aeron_file=.*";

fn main() {
  let args: Vec<String> = env::args().collect();
}

fn get_aeron_file(arg: &str) -> Option<&str> {
  lazy_static! {
    static ref mode: Regex = Regex::new(aeron_regex).unwrap();
  }
  // error: look-around, including look-ahead and look-behind, is not supported
  if mode.is_match(arg) {
    let split: Vec<&str> = arg.split('=').collect();
    let last: Option<&&str> = split.last();
    if last.is_some() && !last.unwrap().is_empty() {
      return Some(last.unwrap());
    }
  }
  None
}

#[cfg(test)]
mod tests {
  use crate::get_aeron_file;

  #[test]
  fn get_aeron_file_test() {
    assert_eq!(get_aeron_file("aeron_file=pom.dat"), Some("pom.dat"));
    assert_eq!(get_aeron_file("aeron_file="), None);
    assert_eq!(get_aeron_file("pom=pouet"), None);
  }
}