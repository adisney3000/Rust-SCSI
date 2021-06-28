use crate::sense::Sense;

/// SSC-4 Section 5.2
#[derive(Default, Debug)]
pub struct Erase16 {
  pub fcs: bool,
  pub lcs: bool,
  pub immed: bool,
  pub long: bool,
  pub method: u8,
  pub smd: bool,
  pub vcm: bool,
  pub partition: u8,
  pub logical_object_identifier: u64,
}

impl Erase16 {
  const OP_CODE: u8 = 0x93;

  pub fn new() -> Erase16 {
    Default::default()
  }
}

impl crate::NoIO for Erase16 {
}

impl crate::Command for Erase16 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 16];

    if !(0..0x04).contains(&self.method) {
      return Err("Method must be in the range 0..4");
    }

    data[0] = Self::OP_CODE;
    data[1] =
      if self.fcs   { 0x8 } else { 0x0 } |
      if self.lcs   { 0x4 } else { 0x0 } |
      if self.immed { 0x2 } else { 0x0 } |
      if self.long  { 0x1 } else { 0x0 };
    data[2] = 
      self.method << 4 |
      if self.smd { 0x2 } else { 0x0 } |
      if self.vcm { 0x1 } else { 0x0 };
    data[3] = self.partition;
    data[4..12].copy_from_slice(&self.logical_object_identifier.to_be_bytes());

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
