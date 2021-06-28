use crate::sense::Sense;

/// SSC-4 Section 6.9
#[derive(Default, Debug)]
pub struct WriteFilemarks6 {
  pub immed: bool,
  pub filemark_count: u32,
}

impl WriteFilemarks6 {
  const OP_CODE: u8 = 0x10;

  pub fn new() -> WriteFilemarks6 {
    Default::default()
  }
}

impl crate::NoIO for WriteFilemarks6 {
}

impl crate::Command for WriteFilemarks6 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    if !(0..(2_u32.pow(24))).contains(&self.filemark_count) {
      return Err("Filemark count must be in the range 0..2^24");
    }

    data[0] = Self::OP_CODE;
    data[1] = if self.immed { 0x1 } else { 0x0 };
    data[2..5].copy_from_slice(&self.filemark_count.to_be_bytes()[1..]);

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
