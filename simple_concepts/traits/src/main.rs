trait Vehicle
{
  fn drive(&self) -> String;
}

struct Car;
struct Ev;
struct Bicycle;

impl Vehicle for Car
{
  fn drive(&self) -> String
  {
    return String::from("vroom vroom");
  }
}

impl Vehicle for Ev
{
  fn drive(&self) -> String
  {
    String::from("quack quack!")
  }
}

impl Vehicle for Bicycle
{
  fn drive(&self) -> String
  {
    String::from("pling plong!")
  }
}

fn main() {
  let car: Car = Car;
  println!("Car says {}", car.drive());

  let ev: Ev = Ev;
  println!("Electric car says {}", ev.drive());

  let bicycle: Bicycle = Bicycle;
  println!("Bicycle says {}", bicycle.drive());
}
