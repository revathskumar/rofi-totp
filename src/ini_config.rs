use ini::Ini;
use totp;
use dir;

pub fn get_list() -> Result<Vec<std::string::String>, String> {

  let mut final_otps : Vec<std::string::String> = Vec::new();

  let home_dir = match dir::home() {
    Some(home) => home,
    None => {
      return Err(String::from("Can't find the HOME directory"));
    }
  };

  let ini_config_path = home_dir.join(".gauth");

  let conf = match Ini::load_from_file(ini_config_path) {
    Ok(c) => c,
    Err(_) => {
      return Err(String::from("Make sure you have .gauth config file in home folder"));
    }
  };

  for (label, prop) in &conf {
    for (_, secret) in prop {
      let otp = totp::generate(secret).unwrap();
      final_otps.push(vec![String::from(label.as_ref().unwrap()), String::from(&otp.to_string())].join(" :: "))
    }
  }

  Ok(final_otps)

}