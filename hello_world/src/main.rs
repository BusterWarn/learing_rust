mod fibonacci;

fn main()
{
  println!("Hello {}!", get_world_str());

  let n:u32 = 20;
  println!("Fibbonacci of {}: {}", n, fibonacci::fibonacci(n));
}

fn get_world_str() -> String
{
  let mut world_str = String::from("W");

  world_str.push_str("orld");

  return world_str
}