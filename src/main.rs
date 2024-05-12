use std::io::Read;

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, ContextBuilder, GameResult};
use regex::Regex;

mod fonts;
mod json_type;

fn main() {
  let (mut ctx, event_loop) = ContextBuilder::new("Text Game From", "gmae199boy")
    .build()
    .expect("aieee, could not create ggez context!");

  let my_game = TextGame::new(&mut ctx);

  event::run(ctx, event_loop, my_game.unwrap());
}

struct TextGame {}

impl TextGame {
  pub fn new(ctx: &mut Context) -> GameResult<TextGame> {
    ctx.gfx.add_font(
      "Tokki",
      graphics::FontData::from_path(ctx, "/font/tokki.ttf")?,
    );

    // fonts::TextParser::new().parse();
    let mut buffer = Vec::new();
    let mut file = ctx.fs.open("/story.json")?;
    file.read_to_end(&mut buffer)?;
    println!(
      "Read from test file: {:?}",
      std::str::from_utf8(&buffer).unwrap()
    );

    let person: json_type::StoryJson = serde_json::from_str(std::str::from_utf8(&buffer).unwrap())
      .expect("JSON was not well-formatted");
    println!("{:?}", person);

    Ok(TextGame {})
  }
}

impl EventHandler for TextGame {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
    canvas.finish(ctx)
  }
}
