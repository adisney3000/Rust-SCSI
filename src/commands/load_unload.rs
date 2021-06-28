use crate::sense::Sense;

/// SSC-4 Section 7.3
#[derive(Default, Debug)]
pub struct LoadUnload {
  pub immed: bool,
  pub hold: bool,
  pub eot: bool,
  pub reten: bool,
  pub load: bool,
}

impl LoadUnload {
  const OP_CODE: u8 = 0x1B;
  
  pub fn new() -> LoadUnload {
    Default::default()
  }
}

impl crate::NoIO for LoadUnload {
}

impl crate::Command for LoadUnload {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    data[0] = Self::OP_CODE;
    data[1] = if self.immed { 0x1 } else { 0x0 };
    data[4] =
        if self.hold  { 0x8 } else { 0x0 } |
        if self.eot   { 0x4 } else { 0x0 } |
        if self.reten { 0x2 } else { 0x0 } |
        if self.load  { 0x1 } else { 0x0 };

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
