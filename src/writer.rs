use std::fs;
use toml::{map::Map, Value}; // 0.5.1

// pub fn to_toml(v: Vec<(String, (String, u32))>) -> Value {
//     let mut servers = Map::new();
//     for (name, (ip_addr, port)) in v {
//         let mut server = Map::new();
//         server.insert("Ipaddr".into(), Value::String(ip_addr));
//         server.insert("Port no".into(), Value::Integer(port as i64));
//         servers.insert(name, Value::Table(server));
//     }

//     let mut map = Map::new();
//     map.insert("server".into(), Value::Table(servers));
//     Value::Table(map)
// }

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

// pub fn write_to_file(s: String) {
//   let v = vec![
//       ("A".into(), ("192.168.4.1".into(), 4476)),
//       ("B".into(), ("192.168.4.8".into(), 1234)),
//       ("C".into(), ("192.168.4.8".into(), 1234))
//   ];

//   let toml_string = toml::to_string(&to_toml(v)).expect("Could not encode TOML value");
//   println!("{}", toml_string);

//   fs::write("servers.toml", toml_string).expect("Could not write to file!");
// }