use std::env;
use std::process;
use std::error::Error;
use std::fs;


struct Config {
  filename: String,
  query: String
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {

    if args.len() < 3 {
      return Err("Too few arguments");
    }
    let filename = args[args.len() - 1].clone();
    let query = args[1].clone();

    Ok(Config {filename, query}) 
  }
}


// returns a type that implements this trait
fn run(config: Config) -> Result<(), Box<dyn Error>>{
  let contents = fs::read_to_string(config.filename)?;

  println!("With text:\n{}", contents);
  Ok(())
}

// add return tpe
fn search(query: &str, contents: &str) ->  {
  vec![]
}


fn main() {

  let args: Vec<String> =
  
   env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
      println!("Problem parsing arguments: {}", err);
      process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

  // run(config);
  if let Err(e) = minigrep::run(config) {
    println!("Application error: {}", e);
    process::exit(1);
  }
}