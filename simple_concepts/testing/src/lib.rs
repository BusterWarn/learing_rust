fn power_of(base: u64, exponent: u16) -> u64
{
  let mut res: u64 = 0;
  for i in 0..=exponent
  {
    match i
    {
      0 => res = 0,
      1 => res = base,
      _ => res = res * base
    }
    println!("i: {}, res: {}", i, res);
  }

  return res;
}

#[cfg(test)]
mod tests {
  use crate::power_of;

  #[test]
  fn power_of_test_1()
  {
    assert_eq!(power_of(2, 2), 4);
  }
  
  #[test]
  fn power_of_test_2()
  {
    assert_eq!(power_of(3, 3), 27);
  }
  
  #[test]
  fn power_of_test_3()
  {
    assert_eq!(power_of(10, 10), 10000000000);
  }
  
  #[test]
  fn power_of_test_4()
  {
    assert_eq!(power_of(0, 9999), 0);
  }
  
  #[test]
  fn power_of_test_5()
  {
    assert_eq!(power_of(1, 9999), 1);
  }
}
