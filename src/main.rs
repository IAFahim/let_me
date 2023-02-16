// use std::thread::sleep;
// use std::time::Duration;
// use inputbot::MouseButton::LeftButton;
// use inputbot::{MouseCursor};
//
// fn main() {
//
//     println!( "\x1B[0mOpening website \x1B[32m\thttps://novalabs.gg/task/index.php/signin");
//     webbrowser::open("https://novalabs.gg/task/index.php/signin").unwrap();
//
//     println!("\x1B[0mTrying to Log-In\x1B[34m\tLeft Screen");
//     sleep(Duration::from_secs(5));
//     MouseCursor::move_abs(-1920/2,455);
//     LeftButton.press();
//     LeftButton.release();
//
//     println!("\x1B[0mTrying to clockIn\x1B[34m\tLeft Screen");
//     sleep(Duration::from_secs(1));
//     MouseCursor::move_abs(-1350,215);
//     LeftButton.press();
//     LeftButton.release();
// }


extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Step {
    start: Start,
    play: Play,
    end: End,
}

#[derive(Debug, Serialize, Deserialize)]
struct Start {
    http_get: String,
    header: String,
    data: String,
    parse: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Play {
    web_browser_open: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct End {
    http_post: String,
    header: String,
    data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Workflow {
    title: String,
    step: Vec<Step>,
}

fn main() {
    let json_str = r#"{
  "title": "Opening website",
  "step": [
    {
      "start": {
        "http::get": "https://novalabs.gg/task/index.php/signin",
        "header": "",
        "data": "",
        "parse":"url"
      },
      "play": {
        "web_browser_open": "https://novalabs.gg/task/index.php/signin/{url}"
      },
      "end": {
        "http::post": "https://novalabs.gg/task/index.php/signin",
        "header": "",
        "data": ""
      }
    }
  ]
}"#;

    let workflow: Workflow = serde_json::from_str(json_str).unwrap();
    println!("{:#?}", workflow);
}