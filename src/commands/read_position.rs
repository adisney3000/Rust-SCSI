use crate::sense::Sense;

/// SSC-4 Section 7.7
#[derive(Default, Debug)]
pub struct ReadPosition {
  pub service_action: u8,
  pub allocation_length: u16,
}

impl ReadPosition {
  pub const SHORT_FORM_BLOCK: u8 = 0x0;
  pub const SHORT_FORM_VENDOR: u8 = 0x1;
  pub const LONG_FORM: u8 = 0x6;
  pub const EXTENDED_FORM: u8 = 0x8;
  const OP_CODE: u8 = 0x34;
  
  pub fn new() -> ReadPosition {
    Default::default()
  }
}

impl crate::Output for ReadPosition {
}

impl crate::Command for ReadPosition {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 10];

    if !(0..32).contains(&self.service_action) {
      return Err("Service action must be in the range 0..32");
    }

    data[0] = Self::OP_CODE;
    data[1] = self.service_action;

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String { 
    "".to_string()
  }
}
