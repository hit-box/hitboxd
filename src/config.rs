use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: String,
}

pub fn read_config(path: &Path) -> Result<Config, serde_yaml::Error> {
    let mut test_yaml = File::open(&path).unwrap();
    let mut s = String::new();
    let _ = test_yaml.read_to_string(&mut s);
    serde_yaml::from_str(s.as_str())
}
