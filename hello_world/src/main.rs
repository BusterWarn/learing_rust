use std::io;

mod fibonacci;

fn main()
{
  println!("Hello {}! Enter a number you'd like to run Fibbonacci with",
           get_world_str());

  let mut number:String = String::new();
  io::stdin()
    .read_line(&mut number)
    .expect("Failed to read input");
  
  while number.ends_with('\r') || number.ends_with('\n')
  {
    number.pop();
  }

  match number.parse::<u32>()
  {
    Ok(n) =>
    {
      println!("Fibbonacci of {}: {}", n, fibonacci::fibonacci(n));
    }
    Err(err) =>
    {
      println!("Could not parse {} as u32. Err: {}", number, err);
    }
  }
}

fn get_world_str() -> String
{
  let mut world_str = String::from("W");

  world_str.push_str("orld");

  return world_str
}