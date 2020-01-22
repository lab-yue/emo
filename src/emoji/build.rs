use regex::Regex;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn main() {
  let out_filename = "./emoji.txt";
  let emoji_data = include_str!("./emoji-test.txt");
  let emoji_re: Regex =
    Regex::new("fully-qualified     # (?P<emoji>.+?) .+? (?P<name>[ \\S]+)").unwrap();
  let captures = emoji_re
    .captures_iter(emoji_data)
    .map(|c| match (&c["emoji"], &c["name"]) {
      (emoji, name) => format!("{}|{}\n", emoji, name),
      _ => "".to_string(),
    })
    .collect::<String>();

  let mut file = match Path::new(out_filename).exists() {
    true => fs::OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(out_filename)
      .expect("Failed to open file"),
    false => fs::File::create(out_filename).expect("Failed to create file"),
  };

  file
    .write_all(&captures.as_bytes())
    .expect("Failed to write file");
}
