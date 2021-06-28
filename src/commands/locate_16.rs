use crate::sense::Sense;

/// SSC-4 Section 7.4
#[derive(Default, Debug)]
pub struct Locate16 {
  pub dest_type: u8,
  pub rsvd: bool,
  pub cp: bool,
  pub immed: bool,
  pub bam: bool,
  pub partition: u8,
  pub logical_object_identifier: u64,
}

impl Locate16 {
  pub const LOGICAL_OBJECT_IDENTIFIER: u8 = 0x0;
  pub const LOGICAL_FILE_IDENTIFIER: u8 = 0x1;
  pub const END_OF_DATA: u8 = 0x3;
  const OP_CODE: u8 = 0x92;

  pub fn new() -> Locate16 {
    Default::default()
  }
}

impl crate::NoIO for Locate16 {
}

impl crate::Command for Locate16 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 16];

    if !(0..8).contains(&self.dest_type) {
      return Err("Dest type must be in the range 0..8");
    }

    data[0] = Self::OP_CODE;
    data[1] =
        self.dest_type << 3 |
        if self.rsvd  { 0x4 } else { 0x0 } |
        if self.cp    { 0x2 } else { 0x0 } |
        if self.immed { 0x1 } else { 0x0 };
    data[2] =
        if self.bam { 0x1 } else { 0x0 };
    data[3] = self.partition;
    data[4..12].copy_from_slice(&self.logical_object_identifier.to_be_bytes());

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
