use std::io::{stdin, stdout, BufRead, Write};

pub fn read() -> Result<u8, ()> {
  let mut s = String::new();
  stdin().lock().read_line(&mut s).unwrap();

  match s.chars().nth(0) {
    Some(c) => Ok(c as u8),
    None => Ok(0u8),
  }
}

pub fn write(character: u8) {
  print!("{}", character as char);
  stdout().flush().unwrap();
}
