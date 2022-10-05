use std::collections::HashMap;

pub fn fibonacci(n: u32) -> u32
{
  let mut memoization:HashMap<u32, u32> = HashMap::new();
  return fib(n, &mut memoization);
}

fn fib(n: u32, mem: &mut HashMap<u32, u32>) -> u32
{
  if n <= 2
  {
    return 1;
  }

  match mem.get(&n)
  {
    Some(res) => {
      return *res;
    },
    None => {
      let fib_1 = fib(n - 1, mem);
      let fib_2 = fib(n  - 2, mem);
      mem.insert(n,  fib_1 + fib_2);
      return fib_1 + fib_2;
    }
  }
}