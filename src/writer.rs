use std::fs;
use toml::{map::Map, Value}; // 0.5.1

pub fn read_current_pod() {
  let contents = fs::read_to_string("current_pod.toml")
    .expect("Something went wrong reading the file");

  println!("{}", contents);
}

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
  let mut aliases = Map::new();
  let mut alias = Map::new();
  alias.insert(alias_name.into(), Value::String(alias_cmd));

  let mut map = Map::new();
  map.insert("alias".into(), Value::Table(alias));
  Value::Table(map)
}

pub fn test_multi() {
  let v = vec![
    ("A".into(), ("192.168.4.1".into(), 4476)),
    ("B".into(), ("192.168.4.8".into(), 1234)),
  ];

  let toml_string = toml::to_string(&to_toml(v)).expect("Could not encode TOML value");
  println!("{}", toml_string);

  fs::write("servers_two.toml", toml_string).expect("Could not write to file!");
}

fn to_toml(v: Vec<(String, (String, u32))>) -> Value {
  let mut servers = Map::new();
  for (name, (ip_addr, port)) in v {
      let mut server = Map::new();
      server.insert("Ipaddr".into(), Value::String(ip_addr));
      server.insert("Port no".into(), Value::Integer(port as i64));
      servers.insert(name, Value::Table(server));
  }

  let mut map = Map::new();
  map.insert("server".into(), Value::Table(servers));
  Value::Table(map)
}