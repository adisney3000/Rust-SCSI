use crate::sense::Sense;

/// SSC-4 Section 7.11
#[derive(Default, Debug)]
pub struct SetCapacity {
  pub immed: bool,
  pub medium_for_proportion_value: u16,
}

impl SetCapacity {
  const OP_CODE: u8 = 0x0B;

  pub fn new() -> SetCapacity {
    Default::default()
  }
}

impl crate::NoIO for SetCapacity {
}

impl crate::Command for SetCapacity {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    data[0] = Self::OP_CODE;
    data[1] = if self.immed { 0x1 } else { 0x0 };
    data[3..5].copy_from_slice(&self.medium_for_proportion_value.to_be_bytes());

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String { 
    "".to_string()
  }
}
