use crate::sense::Sense;

/// SSC-4 Section 6.3
#[derive(Default, Debug)]
pub struct Locate10 {
  pub bt: bool,
  pub cp: bool,
  pub immed: bool,
  pub logical_object_identifier: u32,
  pub partition: u8,
}

impl Locate10 {
  const OP_CODE: u8 = 0x2B;

  pub fn new() -> Locate10 {
    Default::default()
  }
}

impl crate::NoIO for Locate10 {
}

impl crate::Command for Locate10 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 10];

    data[0] = Self::OP_CODE;
    data[1] =
        if self.bt    { 0x4 } else { 0x0 } |
        if self.cp    { 0x2 } else { 0x0 } |
        if self.immed { 0x1 } else { 0x0 };
    data[3..7].copy_from_slice(&self.logical_object_identifier.to_be_bytes());
    data[8] = self.partition;

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
