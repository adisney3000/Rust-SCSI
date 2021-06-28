use crate::sense::Sense;

/// SSC-4 Section 7.8
#[derive(Default, Debug)]
pub struct RecoverBufferedData {
  pub sili: bool,
  pub fixed: bool,
  pub transfer_length: u32,
}

impl RecoverBufferedData {
  const OP_CODE: u8 = 0x14;

  pub fn new() -> RecoverBufferedData {
    Default::default()
  }
}

impl crate::Output for RecoverBufferedData {
}

impl crate::Command for RecoverBufferedData {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    if !(0..(2_u32.pow(24))).contains(&self.transfer_length) {
      return Err("Allow overwrite must be in the range 0..2^24");
    }

    data[0] = Self::OP_CODE;
    data[1] =
        if self.sili  { 0x2 } else { 0x0 } |
        if self.fixed { 0x1 } else { 0x0 };
    data[2..5].copy_from_slice(&self.transfer_length.to_be_bytes()[1..]);

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
