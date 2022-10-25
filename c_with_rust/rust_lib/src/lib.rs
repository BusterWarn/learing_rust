#[no_mangle]
pub extern "C" fn power_of(base: u8, exponent: u8) -> u64
{
  let mut res: u64 = 0;
  for i in 0..=exponent
  {
    match i
    {
      0 => res = 0,
      1 => res = base as u64,
      _ => res = res * base as u64
    }
  }

  return res;
}