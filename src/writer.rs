use std::fs;
use toml::{map::Map, Value}; // 0.5.1

pub fn write_current_pod(pod_name: String, config: String, namespace: String) {
  let toml_string = toml::to_string(&current_pod_to_toml(pod_name, config, namespace)).expect("Could not encode TOML value");
  println!("{}", toml_string);

  fs::write("current_pod.toml", toml_string).expect("Could not write to file!");
}

fn current_pod_to_toml(pod_name: String, config: String, namespace: String) -> Value {
  let mut current_pod = Map::new();
  current_pod.insert("pod_name".into(), Value::String(pod_name));
  current_pod.insert("config".into(), Value::String(config));
  current_pod.insert("namespace".into(), Value::String(namespace));

  let mut map = Map::new();
  map.insert("current_pod".into(), Value::Table(current_pod));
  Value::Table(map)
}

pub fn write_alias(alias_name: String, alias_cmd: String) {
  let toml_string = toml::to_string(&alias_to_toml(alias_name, alias_cmd)).expect("Could not encode TOML value");
  println!("{}", toml_string);

  fs::write("aliases.toml", toml_string).expect("Could not write to file!");
}

fn alias_to_toml(alias_name: String, alias_cmd: String) -> Value {
  let mut alias = Map::new();
  alias.insert("alias_name".into(), Value::String(alias_name));
  alias.insert("alias_cmd".into(), Value::String(alias_cmd));

  let mut map = Map::new();
  map.insert("alias".into(), Value::Table(alias));
  Value::Table(map)
}
