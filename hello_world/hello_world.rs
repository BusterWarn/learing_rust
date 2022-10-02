
fn main()
{
  println!("Hello {}!", get_world_str());
}

fn get_world_str() -> String
{
  let mut world_str = String::from("W");

  world_str.push_str("orld");

  return world_str
}