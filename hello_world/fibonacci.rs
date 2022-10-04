use std::collections::HashMap;

fn main()
{
  let mut memoization:HashMap<i32, i32> = HashMap::new();
  let my_int = fibonacci(12, &mut memoization);

  println!("Fibonacci: {}", my_int);
}

fn fibonacci(n: i32, mem: &mut HashMap<i32, i32>) -> i32
{
  if n <= 2
  {
    return 1;
  }

  match mem.get(&n) {
    Some(res) => {
      return *res;
    },
    None => {
      let fib_1 = fibonacci(n - 1, mem);
      let fib_2 = fibonacci(n  - 2, mem);
      mem.insert(n,  fib_1 + fib_2);
      return fib_1 + fib_2;
    }
  }
/*
  if mem.contains_key(&n)
  {
    return mem.get(&n).unwrap().clone();
  }

  let fib_1 = fibonacci(n - 1, mem);
  let fib_2 = fibonacci(n  - 2, mem);
  mem.insert(n,  fib_1 + fib_2);

  return *mem.get(&n).unwrap_or(0).clone(); */
}