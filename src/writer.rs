use std::fs;
use toml::{map::Map, Value}; // 0.5.1

pub fn read_current_pod() {
  let contents = fs::read_to_string("current_pod.toml")
    .expect("Something went wrong reading the file");

  println!("{}", contents);
}

pub fn write_current_pod(pod_name: String) {
  let toml_string = toml::to_string(&current_pod_to_toml(pod_name)).expect("Could not encode TOML value");
  println!("{}", toml_string);

  fs::write("current_pod.toml", toml_string).expect("Could not write to file!");
}

fn current_pod_to_toml(pod_name: String) -> Value {
  let mut current_pod = Map::new();
  current_pod.insert("pod".into(), Value::String(pod_name));

  let mut map = Map::new();
  map.insert("current_pod".into(), Value::Table(current_pod));
  Value::Table(map)
}
