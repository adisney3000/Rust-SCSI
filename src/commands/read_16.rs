use crate::sense::Sense;

/// SSC-4 Section 5.3
#[derive(Default, Debug)]
pub struct Read16 {
  pub sili: bool,
  pub fixed: bool,
  pub partition: u8,
  pub logical_object_identifier: u64,
  pub transfer_length: u32,
}

impl Read16 {
  const OP_CODE: u8 = 0x88;

  pub fn new() -> Read16 {
    Default::default()
  }
}

impl crate::Output for Read16 {
}

impl crate::Command for Read16 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 16];

    if !(0..(2_u32.pow(24))).contains(&self.transfer_length) {
      return Err("Transfer length must be in the range 0..2^24");
    }

    data[0] = Self::OP_CODE;
    data[1] =
        if self.sili  { 0x2 } else { 0x0 } |
        if self.fixed { 0x1 } else { 0x0 };
    data[3] = self.partition;
    data[4..12].copy_from_slice(&self.logical_object_identifier.to_be_bytes());
    data[12..15].copy_from_slice(&self.transfer_length.to_be_bytes()[1..]);

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
