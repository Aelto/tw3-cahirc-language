use std::collections::HashMap;
use std::path::Path;

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
  let default_path = ".".to_string();
  let args: Vec<String> = std::env::args().collect();
  let cwd = Path::new(args.get(1).unwrap_or(&default_path));
  let config_path = cwd.join("cahirc.toml");
  let content = std::fs::read_to_string(config_path)?;

  let mut config: Config = toml::from_str(&content)?;

  config.package.src = cwd.join(config.package.src).to_str().unwrap().to_string();
  config.package.dist = cwd.join(config.package.dist).to_str().unwrap().to_string();

  let keys: Vec<String> = config.dependencies.keys().map(String::to_string).collect();
  for dep_name in keys {
    let dep_path = config.dependencies.get(&dep_name).unwrap();

    config.dependencies.insert(
      dep_name.to_string(),
      cwd.join(dep_path).to_str().unwrap().to_string(),
    );
  }

  Ok(config)
}
