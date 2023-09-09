// Random Hex String
// By: $t@$h, QVLx Labs

use std::env;
use std::process::Command;

fn main() {
  let args: Vec<String> = env::args().collect();
  let usage = "Random Hex String tool arguments:\n\tLength in bits.\n";
  
  // Input validation
  if args.len() < 2 {
    println!("Missing arguments.");
    println!("{}", usage);
    return;
  }
  else if args.len() > 3 {
    println!("Too many arguments.");
    println!("{}", usage);
    return;
  }

  let output_sh = Command::new("xxd")
                          .arg("-u")
                          .arg("-l")
                          .arg(&args[1].to_string())
                          .arg("-ps")
                          .arg("/dev/urandom")
                          .output()
                          .expect("failed to execute process");

  let outstr = String::from_utf8_lossy(&output_sh.stdout);
  print!("Random string:\n{}", outstr);
}
