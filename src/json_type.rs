use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StoryJson {
  character: Vec<Character>,
}

#[derive(Debug, Deserialize)]
struct Character {
  name: String,
  intro: Vec<String>,
  events: Vec<Event>,
  stories: Vec<Story>,
  endings: Vec<Ending>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
  name: String,
  text: String,
  choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
  text: String,
  next: String,
  reward: String,
}

#[derive(Debug, Deserialize)]
pub struct Story {
  part: String,
  text: String,
  next_story: String,
}

#[derive(Debug, Deserialize)]
pub struct Ending {
  event_result: Vec<String>,
  name: String,
  text: String,
}

// {
//   "character": [
//     {
//       "name": "명길",
//       "intro": [
//         "안녕하세요. 저는 명길입니다. 잘 부탁드립니다."
//       ],
//       "events": [
//         {
//           "name": "1",
//           "text": "aaa",
//           "choices": [
//             {
//               "text": "1",
//               "next": "1",
//               "reward": "1"
//             },
//             {
//               "text": "2",
//               "next": "2",
//               "reward": "1"
//             }
//           ]
//         }
//       ],
//       "stories": [
//         {
//           "part": "1",
//           "text": "파트1 스토리 텍스트",
//           "next_story": "1"
//         },
//         {
//           "part": "2",
//           "text": "파트2 스토리 텍스트",
//           "next_story": "1"
//         }
//       ],
//       "endings": [
//         {
//           "event_result": [
//             "1",
//             "2",
//             "3"
//           ],
//           "name": "1",
//           "text": "text"
//         }
//       ]
//     }
//   ]
// }
