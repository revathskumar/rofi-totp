extern crate yaml_rust;
extern crate ini;
extern crate dirs;

use std::fs::File;
use std::error::Error;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use yaml_rust::{YamlLoader};

mod totp;
mod ini_config;

fn main() {
  let mut is_error = false;
  let mut error_message = "";

  let final_otps = match ini_config::get_list() {
    Ok(c) => c,
    Err(why) => {
      error_message = why;
      is_error = true;
      println!("in Err :: {:?} :: {}", error_message, is_error);
      Vec::new()
    },
  };

  if is_error {


    println!("in is_error :: {:?} :: {}", error_message, is_error);
      

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
        stdin.write_all(error_message.as_bytes()).unwrap();
      }

    // let rofi_status = rofi.wait_with_output().unwrap();
    // return panic!(error_message);
  }

  // let config_path = match dirs::home_dir() {
  //   Some(x) => x.join("2fa.yml"),
  //   None => panic!("Can't find the HOME directory"),
  // };

  // let mut config_file = match File::open(&config_path) {
  //   Err(why) => panic!("can't open file {}: {}", config_path.display(), why.description()),
  //   Ok(file) => file,
  // };

  // let mut s = String::new();
  // let content = match config_file.read_to_string(&mut s) {
  //   Err(why) => panic!("can't read file {}: {}", config_path.display(), why.description()),
  //   Ok(_) => s,
  // };

  // let config = YamlLoader::load_from_str(&content).unwrap();

  // let apps = &config[0]["apps"].as_vec().unwrap();

  
  
  // for app in apps.iter() {
  //   let label = app["label"].as_str().unwrap();
  //   let secret = app["secret"].as_str().unwrap();
  //   let otp = totp::generate(secret).unwrap();
  //   final_otps.push(vec![label, &otp.to_string()].join(" :: "))
  // }

  let xdotool = Command::new("xdotool")
    .arg("getactivewindow")
    .output()
    .unwrap();

  let window_id = String::from_utf8_lossy(&xdotool.stdout);

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
    let option_parts: Vec<&str>  = selected_option.split(" :: ").collect();

    let otp = option_parts[1];

    let mut _sleep = Command::new("sleep")
      .arg("0.5")
      .output()
      .unwrap();

    let mut _xdo = Command::new("xdotool")
      .arg("windowactivate")
      .arg(window_id.to_string())
      .output()
      .unwrap();

    let mut _type = Command::new("xdotool")
      .arg("type")
      .arg(otp)
      .output()
      .unwrap();
  }
}
