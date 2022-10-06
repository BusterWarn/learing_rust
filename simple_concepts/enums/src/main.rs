use std::fmt;

enum Rat
{
  WCDMA (char),
  LTE (u64, u64),
  NR (u64, u64)
}

impl fmt::Display for Rat {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
  {
    match self
    {
      Rat::WCDMA(property) => write!(f, "WCDMA properties: {}", property),
      Rat::LTE(freq_from, freq_to) => write!(f, "LTE with frequence {} - {}", freq_from, freq_to),
      Rat::NR(freq_from, freq_to) => write!(f, "NR with frequence {} - {}", freq_from, freq_to)
    }
  }
}

fn main()
{
  let wcdma_rat = Rat::WCDMA(char::from_u32(0x1F422).expect("Not a valid code point"));
  let lte_rat = Rat::LTE(20000, 100000);
  let nr_rat = Rat::NR(100000, 200000);
  println!("{}\n{}\n{}", wcdma_rat.to_string(), lte_rat.to_string(), nr_rat.to_string());
}
