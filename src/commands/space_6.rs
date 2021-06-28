use crate::sense::Sense;

/// SSC-4 Section 6.6
#[derive(Default, Debug)]
pub struct Space6 {
  pub code: u8,
  pub count: u32,
}

impl Space6 {
  pub const LOGICAL_BLOCKS: u8 = 0x0;
  pub const FILEMARKS: u8 = 0x1;
  pub const SEQUENTIAL_FILEMARKS: u8 = 0x2;
  pub const END_OF_DATA: u8 = 0x3;
  const OP_CODE: u8 = 0x11;
  
  pub fn new() -> Space6 {
    Default::default()
  }
}

impl crate::NoIO for Space6 {
}

impl crate::Command for Space6 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    if !(0..16).contains(&self.code) {
      return Err("Code must be in the range 0..16");
    }

    if !(0..(2_u32.pow(24))).contains(&self.count) {
      return Err("Count must be in the range 0..2^24");
    }

    data[0] = Self::OP_CODE;
    data[1] = self.code;
    data[2..5].copy_from_slice(&self.count.to_be_bytes()[1..]);

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
