use crate::sense::Sense;

/// SSC-4 Section 7.9
#[derive(Default, Debug)]
pub struct ReportDensitySupport {
  pub medium_type: bool,
  pub media: bool,
  pub allocation_length: u16,
}

impl ReportDensitySupport {
  const OP_CODE: u8 = 0x44;

  pub fn new() -> ReportDensitySupport {
    Default::default()
  }
}

impl crate::Output for ReportDensitySupport {
}

impl crate::Command for ReportDensitySupport {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 10];

    data[0] = Self::OP_CODE;
    data[1] =
        if self.medium_type { 0x2 } else { 0x0 } |
        if self.media       { 0x1 } else { 0x0 };
    data[7..9].copy_from_slice(&self.allocation_length.to_be_bytes());

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
