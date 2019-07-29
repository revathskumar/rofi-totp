use ini::Ini;
use totp;

pub fn get_list() -> Result<Vec<std::string::String>, &'static str> {

  let mut final_otps : Vec<std::string::String> = Vec::new();

  
  let ini_config_path = match dirs::home_dir() {
    Some(x) => x.join(".gauth"),
    None => {
      return Err("Can't find the HOME directory");
    }
  };

  let conf = match Ini::load_from_file(ini_config_path) {
    Ok(c) => c,
    Err(_) => {
      return Err("Make sure you have .gauth config file in home folder");
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