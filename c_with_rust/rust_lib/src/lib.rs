#[no_mangle]
pub extern fn power_of(base: u64, exponent: u16) -> u64
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
  }

  return res;
}