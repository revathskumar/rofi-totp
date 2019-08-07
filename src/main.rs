extern crate yaml_rust;
extern crate ini;

use std::process::{Command};

mod totp;
mod ini_config;
mod yaml_config;
mod rofi;

fn main() {
  let mut is_error = false;
  let mut error_message = "";

  let mut final_otps = match ini_config::get_list() {
    Ok(c) => c,
    Err(why) => {
      error_message = why;
      is_error = true;
      Vec::new()
    },
  };

  if is_error {
    final_otps = match yaml_config::get_list() {
    Ok(c) => {
      is_error = false;
      error_message = "";  
      c
    },
    Err(why) => {
      error_message = why;
      is_error = true;
      Vec::new()
    },
  };
  }

  if is_error {
    let rf = rofi::create();
    rofi::set_content(rf, error_message);
    return
  }

  let xdotool = Command::new("xdotool")
    .arg("getactivewindow")
    .output()
    .unwrap();

  let window_id = String::from_utf8_lossy(&xdotool.stdout);

  let mut rf = rofi::create();
  rf = rofi::set_content(rf, &final_otps.join("\n"));

  let rofi_status = rf.wait_with_output().unwrap();
  
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
