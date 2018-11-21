extern crate yaml_rust;
extern crate dirs;

use std::fs::File;
use std::error::Error;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use yaml_rust::{YamlLoader};

mod totp;

fn main() {

  let config_path = match dirs::home_dir() {
    Some(x) => x.join("2fa.yml"),
    None => panic!("Can't find the HOME directory"),
  };

  let mut config_file = match File::open(&config_path) {
    Err(why) => panic!("can't open file {}: {}", config_path.display(), why.description()),
    Ok(file) => file,
  };

  let mut s = String::new();
  let content = match config_file.read_to_string(&mut s) {
    Err(why) => panic!("can't read file {}: {}", config_path.display(), why.description()),
    Ok(_) => s,
  };

  let config = YamlLoader::load_from_str(&content).unwrap();

  let apps = &config[0]["apps"].as_vec().unwrap();

  let mut final_otps = Vec::new();
  
  for app in apps.iter() {
    let label = app["label"].as_str().unwrap();
    let secret = app["secret"].as_str().unwrap();
    let otp = totp::generate(secret).unwrap();
    final_otps.push(vec![label, &otp.to_string()].join(" :: "))
  }

  let mut rofi = Command::new("rofi")
        .arg("-dmenu")
        .arg("-p")
        .arg("2fa")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute rofi command");

  {
    let stdin = rofi.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(final_otps.join("\n").as_bytes()).unwrap();
  }


  let rofi_status = rofi.wait_with_output().unwrap();
  
  if rofi_status.status.success() {
    let selected_option = String::from_utf8_lossy(&rofi_status.stdout);

    println!("stdout :: {:#?}",  selected_option.trim());
    println!("status :: {:#?}", rofi_status.status.success())

  }
}
