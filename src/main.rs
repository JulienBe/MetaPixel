use regex::Regex;
use std::env;
use bevy::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  App::build()
    .add_plugins(DefaultPlugins)
    .run();
}

fn is_valid_image_path(arg: &str) -> bool {
  return Regex::new(r"\.png$").unwrap().is_match(&arg)
}

#[cfg(test)]
mod tests {
  use crate::is_valid_image_path;

  #[test]
  fn is_valid_image_path_test() {
    assert_eq!(is_valid_image_path("pom"), false);
    assert_eq!(is_valid_image_path(""), false);
    assert_eq!(is_valid_image_path("/home/julien/image.png"), true);
  }
}