use std::fs;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct CurrentPodConfig {
  current_pod: Option<ServerConfig>
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    config: Option<String>,
    namespace: Option<String>,
    pod_name: Option<String>
}

#[derive(Debug, Deserialize)]
struct AliasConfig {
  alias: Option<AliasCommand>
}

#[derive(Debug, Deserialize)]
struct AliasCommand {
    alias_name: Option<String>,
    alias_cmd: Option<String>
}

pub fn read_current_pod() {
  let contents = fs::read_to_string("current_pod.toml")
    .expect("Something went wrong reading the file");

  let decoded: CurrentPodConfig = toml::from_str(&contents).unwrap();
  println!("{:#?}", decoded);
}

pub fn read_current_access_command() {
  let current_pod = fs::read_to_string("current_pod.toml")
    .expect("Something went wrong reading the file");

  let decoded: CurrentPodConfig = toml::from_str(&current_pod).unwrap();

  let current_pod_hash: ServerConfig = decoded.current_pod.unwrap();

  let current_config = current_pod_hash.config.unwrap();
  let current_namespace = current_pod_hash.namespace.unwrap();
  let current_pod_name = current_pod_hash.pod_name.unwrap();

  let aliases = fs::read_to_string("aliases.toml")
    .expect("Something went wrong reading the file");

  let decoded_alias: AliasConfig = toml::from_str(&aliases).unwrap();
  let alias_hash: AliasCommand = decoded_alias.alias.unwrap();
  let _alias_name = alias_hash.alias_name.unwrap();
  let alias_cmd = alias_hash.alias_cmd.unwrap();

  println!("kubectl --kubeconfig={} -n {} exec -it {} -- {}", current_config, current_namespace, current_pod_name, alias_cmd);
}
