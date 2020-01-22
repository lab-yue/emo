#[derive(Debug)]
pub struct Emoji {
  pub icon: String,
  pub name: String,
}

pub fn search(text: String) -> Vec<Emoji> {
  let emoji_source = include_str!("./emoji.txt");
  emoji_source
    .split("\n")
    .map(|emoji| {
      let mut parts = emoji.split("|");
      match (parts.nth(0), parts.last()) {
        (Some(icon), Some(name)) => Emoji {
          icon: icon.to_string(),
          name: name.to_string(),
        },
        _ => {
          println!("{:#?}", emoji);
          panic!()
        }
      }
    })
    .filter(|emoji| emoji.name.contains(&text))
    .collect::<Vec<Emoji>>()
}
