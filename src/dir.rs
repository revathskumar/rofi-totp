use std::path::{PathBuf};
use std::env;

pub fn home()-> Option<PathBuf> {
  let user = env::var_os("USER").unwrap();
  let user_str = user.to_str().unwrap();

  let mut path_str: String = String::from("/home/");
  path_str.push_str(user_str);

  let path = PathBuf::from(path_str);

  if path.is_dir() {
    return Some(path)
  }
  return None
}