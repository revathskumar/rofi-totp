use std::fs::File;
use std::io::{Read};
use yaml_rust::{YamlLoader};

use totp;
use dir;

pub fn get_list() -> Result<Vec<std::string::String>, &'static str> {

  let mut final_otps : Vec<std::string::String> = Vec::new();
  
  let home_dir = match dir::home() {
    Some(home) => home,
    None => {
      return Err("Can't find the HOME directory");
    }
  };

  let config_path = home_dir.join("2fa.yml");

  let mut config_file = match File::open(&config_path) {
    Err(_) => {
      return Err("Make sure you have .gauth (ini format) or 2fa.yml config file in home folder");
    }
    Ok(file) => file,
  };

  let mut s = String::new();
  let content = match config_file.read_to_string(&mut s) {
    Err(_) => {
      return Err("Make sure you have .gauth (ini format) or 2fa.yml config file in home folder");
    }Ok(_) => s,
  };

  let config = YamlLoader::load_from_str(&content).unwrap();

  let apps = &config[0]["apps"].as_vec().unwrap();
  
  for app in apps.iter() {
    let label = app["label"].as_str().unwrap();
    let secret = app["secret"].as_str().unwrap();
    let otp = totp::generate(secret).unwrap();
    final_otps.push(vec![label, &otp.to_string()].join(" :: "))
  }

  Ok(final_otps)

}