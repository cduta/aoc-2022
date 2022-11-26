use std::env;
use std::io;
use std::path;

fn pathbuf_to_string(buf: io::Result<path::PathBuf>) -> Result<String, io::Error> {
  buf.and_then(
    |buf| buf.to_str().ok_or(io::Error::new(io::ErrorKind::Other, "to_str() failed.")).and_then(
      |str| Ok(str.to_string())))
}

fn print_path(path: Result<String, io::Error>) {
  match path {
    Ok(p)  => println!("Hello, world from {}!", p),
    Err(e) => println!("Error when reading current dir: {}", e.to_string())
  }
}

fn main() {
  print_path(pathbuf_to_string(Err(io::Error::new(io::ErrorKind::Other, "Could not read current dir."))));
  print_path(pathbuf_to_string(env::current_dir()));
}
