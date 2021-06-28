use crate::sense::Sense;

/// SSC-4 Section 7.5
#[derive(Default, Debug)]
pub struct PreventAllowMediumRemoval {
  pub prevent: u8,
}

impl PreventAllowMediumRemoval {
  pub const ALLOWED: u8 = 0x0;
  pub const PREVENTED: u8 = 0x1;
  const OP_CODE: u8 = 0x1E;
  
  pub fn new() -> PreventAllowMediumRemoval {
    Default::default()
  }
}

impl crate::NoIO for PreventAllowMediumRemoval {
}

impl crate::Command for PreventAllowMediumRemoval {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    if !(0..4).contains(&self.prevent) {
      return Err("Code must be in the range 0..4");
    }

    data[0] = Self::OP_CODE;
    data[4] = self.prevent;

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
