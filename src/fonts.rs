use std::collections::BTreeMap;

use ggez::graphics::Text;
use regex::Regex;

use crate::json_type::StoryJson;

// tag 정의
// <c:COLOR_CODE text> 색상을 지정할 수 있습니다.
// <s:SIZE text> 폰트 크기를 지정할 수 있습니다.
// <f:FONT_NAME text> 폰트를 지정할 수 있습니다.
// <a:EASING text> 애니메이션을 지정할 수 있습니다.

pub struct TextParser {}

impl TextParser {
  pub fn new() -> Self {
    TextParser {}
  }

  pub fn parse<T>(&mut self, text: &T) {
    let mut story: BTreeMap<&str, T> = BTreeMap::new();
    // story.insert(
    //   "1",
    //   "asdasd
    // asdasd
    // asdasd
    // ",
    // );

    let re = Regex::new(r"<(.*?)>").unwrap();
    let caps = re.captures_iter("hool <asdasd>,<qqq><aaa>skdi<zzz>");
    // println!(
    //   "{:?}",
    //   caps
    //     .map(|cap| cap.get(1).unwrap().as_str())
    //     .collect::<Vec<_>>()
    // );
  }
}

pub struct StoryManager {
  text: Text,
}

pub struct TextManager<'a> {
  font_name: &'a str,
  text: Text,
}

impl<'a> TextManager<'a> {
  pub fn new(font_name: &'a str) -> Self {
    TextManager {
      font_name,
      text: Text::new(""),
    }
  }

  pub fn add_plain_text(&mut self, text: &str) {
    self.text.add(text);
  }
}
