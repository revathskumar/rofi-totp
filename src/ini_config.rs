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

  print!("Plist path :: {:?}", ini_config_path);

  let conf = match Ini::load_from_file(ini_config_path) {
    Ok(c) => c,
    Err(why) => {
      return Err("Make sure you have .gauth config file in home folder");
    }
  };

  for (label, prop) in &conf {
    println!("Section: {:?}", label);
    for (key, secret) in prop {
      println!("{:?}:{:?}", key, secret);
      let otp = totp::generate(secret).unwrap();
      println!("otp :: {:?}", otp);
      final_otps.push(vec![String::from(label.as_ref().unwrap()), String::from(&otp.to_string())].join(" :: "))
    }
  }

  Ok(final_otps)

}