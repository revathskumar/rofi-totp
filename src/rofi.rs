use std::io::{Write};
use std::process::{Command, Stdio};

pub fn create() -> std::process::Child {
  return Command::new("rofi")
    .arg("-dmenu")
    .arg("-p")
    .arg("2fa")
    .arg("-i")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to execute rofi command")
}

pub fn set_content(mut rofi: std::process::Child, content: &str) -> std::process::Child {
  let stdin = rofi.stdin.as_mut().expect("Failed to open stdin");
  stdin.write_all(content.as_bytes()).unwrap();
  rofi
}
