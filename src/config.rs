use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
  pub package: ConfigPackage,
  pub dependencies: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigPackage {
  pub name: String,
  pub src: String,
  pub dist: String,
}

pub fn read_config() -> std::io::Result<Config> {
  let config_path = std::path::Path::new("cahirc.toml");
  let content = std::fs::read_to_string(config_path)?;

  Ok(toml::from_str(&content)?)
}
