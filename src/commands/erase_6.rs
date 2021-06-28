use crate::sense::Sense;

/// SSC-4 Section 6.2
#[derive(Default, Debug)]
pub struct Erase6 {
  pub immed: bool,
  pub long: bool,
  pub method: u8,
  pub smd: bool,
  pub vcm: bool,
}

impl Erase6 {
  const OP_CODE: u8 = 0x19;

  pub fn new() -> Erase6 {
    Default::default()
  }
}

impl crate::NoIO for Erase6 {
}

impl crate::Command for Erase6 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    if !(0..0x04).contains(&self.method) {
      return Err("Method must be in the range 0..4");
    }

    data[0] = Self::OP_CODE;
    data[1] =
        if self.immed { 0x2 } else { 0x0 } |
        if self.long  { 0x1 } else { 0x0 };
    data[2] =
        self.method << 4 |
        if self.smd { 0x2 } else { 0x0 } |
        if self.vcm { 0x1 } else { 0x0 };

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
