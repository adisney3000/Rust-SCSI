use crate::sense::Sense;

/// SSC-4 Section 6.7
#[derive(Default, Debug)]
pub struct Verify6 {
  pub vte: bool,
  pub vlbpm: bool,
  pub vbf: bool,
  pub immed: bool,
  pub bytcmp: bool,
  pub fixed: bool,
  pub verification_length: u32,
}

impl Verify6 {
  const OP_CODE: u8 = 0x13;
  
  pub fn new() -> Verify6 {
    Default::default()
  }
}

impl crate::NoIO for Verify6 {
}

impl crate::Command for Verify6 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    if !(0..(2_u32.pow(24))).contains(&self.verification_length) {
      return Err("Verification length must be in the range 0..2^24");
    }

    data[0] = Self::OP_CODE;
    data[1] =
        if self.vte    { 0x20 } else { 0x0 } |
        if self.vlbpm  { 0x10 } else { 0x0 } |
        if self.vbf    { 0x08 } else { 0x0 } |
        if self.immed  { 0x04 } else { 0x0 } |
        if self.bytcmp { 0x02 } else { 0x0 } |
        if self.fixed  { 0x01 } else { 0x0 };
    data[2..5].copy_from_slice(&self.verification_length.to_be_bytes()[1..]);

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
