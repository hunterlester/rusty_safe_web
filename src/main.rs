use std::collections::HashMap;
#[macro_use]
extern crate stdweb;

use stdweb::web::{
    INode,
    document,
    window
};

fn main() {
  let mut hash_map_test = HashMap::new();
  hash_map_test.insert("key_1", "value_1");
  hash_map_test.insert("key_2", "value_2");
  hash_map_test.insert("key_3", "value_3");

  println!("Coming to you from Rust, no FFI's: {:?}", hash_map_test);

  stdweb::initialize();

  let heading = document().query_selector("#heading1").unwrap();
  heading.set_text_content("velkommen emscripten");

  let subtext = document().query_selector("#subtext").unwrap();
  subtext.set_text_content("Hello, I'm coming to you from a Rust program, compiled with Emscripten to target Web Assembly.");

  

  stdweb::event_loop();
}
