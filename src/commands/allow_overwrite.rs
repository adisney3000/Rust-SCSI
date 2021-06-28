use crate::sense::Sense;

/// SSC-4 Section 7.1
#[derive(Default, Debug)]
pub struct AllowOverwrite {
  pub allow_overwrite: u8,
  pub partition: u8,
  pub logical_object_identifier: u64,
}

impl AllowOverwrite {
  pub const DISABLED: u8 = 0x0;
  pub const CURRENT_POSITION: u8 = 0x1;
  pub const FORMAT: u8 = 0x2;
  const OP_CODE: u8 = 0x82;

  pub fn new() -> AllowOverwrite {
    Default::default()
  }
}

impl crate::NoIO for AllowOverwrite {
}

impl crate::Command for AllowOverwrite {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 16];

    if !(0..16).contains(&self.allow_overwrite) {
      return Err("Allow overwrite must be in the range 0..16");
    }

    data[0] = Self::OP_CODE;
    data[2] = self.allow_overwrite;
    data[3] = self.partition;
    data[4..12].copy_from_slice(&self.logical_object_identifier.to_be_bytes());

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
