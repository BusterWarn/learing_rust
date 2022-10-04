use std::collections::HashMap;

fn main()
{
  let memoization:HashMap<i32, i32> = HashMap::new();
  let my_int = fibonacci(12, &memoization);

  println!("Fibonacci: {}", my_int);
}

fn fibonacci(n: i32, mem: &HashMap<i32, i32>) -> &i32
{
  if n <= 2
  {
    return &1;
  }

  //let mut return_value:i32 = 0;
  if mem.contains_key(&n)
  {
    //return_value = 
    return mem.get(&n).unwrap();
  }

  mem.insert(n, fibonacci(n - 1, &mem) + fibonacci(n  - 2, &mem));

  return &5;
  // return *mem.get(&n).unwrap_or(&0);
}