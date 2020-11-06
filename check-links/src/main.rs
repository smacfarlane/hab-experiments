use std::path::Path;
use std::{env,fs};
use goblin::Object;

fn main() {
  let mut args = env::args().into_iter();
  let _ = args.next();
  if let Some(ex) = args.next() {

    let path = Path::new(ex.as_str());
    let buffer = fs::read(path).unwrap();

    match Object::parse(&buffer).unwrap() {
      Object::Elf(elf) => {
        println!("elf: {:#?}", &elf);
      },
      _ => {
        println!("NOPE");
      }
    }
  }
}

