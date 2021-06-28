use crate::sense::Sense;

/// SSC-4 Section 5.7
#[derive(Default, Debug)]
pub struct WriteFilemarks16 {
  pub fcs: bool,
  pub lcs: bool,
  pub immed: bool,
  pub partition: u8,
  pub logical_object_identifier: u64,
  pub filemark_count: u32,
}

impl WriteFilemarks16 {
  const OP_CODE: u8 = 0x80;

  pub fn new() -> WriteFilemarks16 {
    Default::default()
  }
}

impl crate::NoIO for WriteFilemarks16 {
}

impl crate::Command for WriteFilemarks16 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 16];

    if !(0..(2_u32.pow(24))).contains(&self.filemark_count) {
      return Err("Filemark count must be in the range 0..2^24");
    }

    data[0] = Self::OP_CODE;
    data[1] =
        if self.fcs   { 0x8 } else { 0x0 } |
        if self.lcs   { 0x4 } else { 0x0 } |
        if self.immed { 0x1 } else { 0x0 };
    data[3] = self.partition;
    data[4..12].copy_from_slice(&self.logical_object_identifier.to_be_bytes());
    data[12..15].copy_from_slice(&self.filemark_count.to_be_bytes()[1..]);

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
