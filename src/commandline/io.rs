use std::{io::{self, BufRead}, thread};

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
  let handle = thread::spawn(move || {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
      match line {
        Ok(input) => println!("Вы ввели: {}", input),
        Err(error) => eprintln!("Ошибка чтения строки: {}", error),
      }
    }
  });

  handle.join().map_err(|_| {
    Box::new(io::Error::new(io::ErrorKind::Other, "Thread runtime error")) as Box<dyn std::error::Error>
  })?;

  Ok(())
}
