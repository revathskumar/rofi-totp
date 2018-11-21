extern crate base32;
extern crate oath;

// use oath::totp;

pub fn generate(secret: &str) -> Result<u64, &'static str> {
  let secret_bytes = try!(base32::decode(base32::Alphabet::RFC4648 {padding: false}, secret).ok_or("invalid base32"));
  let code: u64 = oath::totp_raw_now(&secret_bytes, 6, 0, 30, &oath::HashType::SHA1);
  Ok(code)
}
