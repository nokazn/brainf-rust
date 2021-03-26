use std::io::{self, Read};

const MEMORY_SIZE: usize = 2048;

fn exec(src: String) -> Result<(), ()> {
  let mut memory = [0; MEMORY_SIZE];
  let mut pointer: usize = 0;
  let mut index: usize = 0;
  while index < src.len() {
    let c = src.chars().nth(index).unwrap();
    match c {
      '>' => pointer += 1,
      '<' => pointer -= 1,
      '+' => memory[pointer] += 1,
      '-' => memory[pointer] -= 1,
      '.' => print!("{}", char::from(memory[pointer])),
      ',' => memory[pointer] = io::stdin().bytes().nth(0).and_then(|r| r.ok()).unwrap(),
      '[' => {
        if memory[pointer] == 0 {
          let mut brackets = 1;
          while brackets > 0 {
            index += 1;
            brackets += match src.chars().nth(index).unwrap() {
              '[' => 1,
              ']' => -1,
              _ => 0,
            }
          }
        };
      }
      ']' => {
        if memory[pointer] != 0 {
          let mut brackets = 1;
          while brackets > 0 {
            index -= 1;
            brackets += match src.chars().nth(index).unwrap() {
              ']' => 1,
              '[' => -1,
              _ => 0,
            }
          }
        }
      }
      _ => (),
    }
    index += 1;
  }

  println!("");
  Ok(())
}

fn input() -> String {
  let mut str = String::new();
  io::stdin().read_line(&mut str).ok();
  str
}

fn main() {
  exec(input()).ok();
}
