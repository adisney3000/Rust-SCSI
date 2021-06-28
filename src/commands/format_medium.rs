use crate::sense::Sense;

/// SSC-4 Section 7.2
#[derive(Default, Debug)]
pub struct FormatMedium {
  pub verify: bool,
  pub immed: bool,
  pub format: u8,
  pub transfer_length: u16,
}

impl FormatMedium {
  pub const DEFAULT: u8 = 0x0;
  pub const PARTITION_VOLUME: u8 = 0x1;
  pub const DEFAULT_THEN_PARTITION: u8 = 0x2;
  const OP_CODE: u8 = 0x04;

  pub fn new() -> FormatMedium {
    Default::default()
  }
}

impl crate::Input for FormatMedium {
}

impl crate::Command for FormatMedium {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    if !(0..16).contains(&self.format) {
      return Err("Format must be in the range 0..16");
    }

    data[0] = Self::OP_CODE;
    data[1] =
        if self.verify { 0x2 } else { 0x0 } |
        if self.immed { 0x1 } else { 0x0 };
    data[2] = self.format;
    data[3..5].copy_from_slice(&self.transfer_length.to_be_bytes());

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
