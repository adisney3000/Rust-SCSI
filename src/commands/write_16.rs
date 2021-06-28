use crate::sense::Sense;

/// SSC-4 Section 5.6
#[derive(Default, Debug)]
pub struct Write16 {
  pub fcs: bool,
  pub lcs: bool,
  pub rsvd: bool,
  pub fixed: bool,
  pub partition: u8,
  pub logical_object_identifier: u64,
  pub transfer_length: u32,
}

impl Write16 {
  const OP_CODE: u8 = 0x8A;

  pub fn new() -> Write16 {
    Default::default()
  }
}

impl crate::Input for Write16 {
}

impl crate::Command for Write16 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 16];

    if !(0..(2_u32.pow(24))).contains(&self.transfer_length) {
      return Err("Transfer length must be in the range 0..2^24");
    }

    data[0] = Self::OP_CODE;
    data[1] =
        if self.fcs   { 0x8 } else { 0x0 } |
        if self.lcs   { 0x4 } else { 0x0 } |
        if self.rsvd  { 0x2 } else { 0x0 } |
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
