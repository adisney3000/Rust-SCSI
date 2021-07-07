use crate::sense::Sense;

/// SSC-4 Section 5.5
#[derive(Default, Debug)]
pub struct Verify16 {
  pub vte: bool,
  pub vlbpm: bool,
  pub vbf: bool,
  pub immed: bool,
  pub bytcmp: bool,
  pub fixed: bool,
  pub partition: u8,
  pub logical_object_identifier: u64,
  pub verification_length: u32,
}

impl Verify16 {
  const OP_CODE: u8 = 0x8F;

  pub fn new() -> Verify16 {
    Default::default()
  }
}

impl crate::Input for Verify16 {
}

impl crate::Command for Verify16 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 16];

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
    data[3] = self.partition;
    data[4..12].copy_from_slice(&self.logical_object_identifier.to_be_bytes());
    data[12..15].copy_from_slice(&self.verification_length.to_be_bytes()[1..]);

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
