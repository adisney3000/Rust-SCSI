use crate::sense::Sense;
use std::convert::TryInto;

/// SSC-4 Section 7.9
#[derive(Default, Debug)]
pub struct ReportDensitySupport {
  pub medium_type: bool,
  pub media: bool,
  pub allocation_length: u16,
}

#[derive(Debug)]
pub enum ReportDensitySupportOutput {
  Density(ReportDensitySupportOutputDensity),
  Medium(ReportDensitySupportOutputMedium),
}

#[derive(Default, Debug)]
pub struct ReportDensitySupportOutputDensity {
  pub primary_density_code: u8,
  pub secondary_density_code: u8,
  pub wrtok: bool,
  pub dup: bool,
  pub deflt: bool,
  //pub dlv: bool,
  //pub descriptor_length: u16,
  pub bits_per_mm: u32,
  pub media_width: u16,
  pub tracks: u16,
  pub capacity: u32,
  pub assigning_organization: String,
  pub density_name: String,
  pub description: String,
}

#[derive(Default, Debug)]
pub struct ReportDensitySupportOutputMedium {
  pub medium_type: u8,
  pub primary_density_codes: Vec <u8>,
  pub media_width: u16,
  pub medium_length: u16,
  pub assigning_organization: String,
  pub medium_type_name: String,
  pub description: String,
}

impl ReportDensitySupport {
  const OP_CODE: u8 = 0x44;
  const DENSITY_SIZE: usize = 52;
  const MEDIUM_SIZE: usize = 56;

  pub fn new() -> ReportDensitySupport {
    Default::default()
  }

  fn parse_density(&self, buf: &[u8]) -> Option <ReportDensitySupportOutputDensity> {
    if buf.len() != Self::DENSITY_SIZE {
      return None;
    }

    Some(ReportDensitySupportOutputDensity {
      primary_density_code: buf[0],
      secondary_density_code: buf[1],
      wrtok: buf[2] & 0x80 == 0x80,
      dup: buf[2] & 0x40 == 0x40,
      deflt: buf[2] & 0x20 == 0x20,
      bits_per_mm: u32::from_be_bytes([0, buf[5], buf[6], buf[7]]),
      media_width: u16::from_be_bytes(buf[8..10].try_into().unwrap()),
      tracks: u16::from_be_bytes(buf[10..12].try_into().unwrap()),
      capacity: u32::from_be_bytes(buf[12..16].try_into().unwrap()),
      assigning_organization: match String::from_utf8(buf[16..24].to_vec()) {
        Ok(s) => { s },
        Err(_) => { "".to_string() },
      },
      density_name: match String::from_utf8(buf[24..32].to_vec()) {
        Ok(s) => { s },
        Err(_) => { "".to_string() },
      },
      description: match String::from_utf8(buf[32..52].to_vec()) {
        Ok(s) => { s },
        Err(_) => { "".to_string() },
      },
    })
  }

  fn parse_medium(&self, buf: &[u8]) -> Option <ReportDensitySupportOutputMedium> {
    if buf.len() != Self::MEDIUM_SIZE {
      return None;
    }

    Some(ReportDensitySupportOutputMedium {
      medium_type: buf[0],
      primary_density_codes: {
        let mut p: Vec <u8> = Vec::new();
        for i in 0..std::cmp::min(buf[4] as usize, 9) {
          p.push(buf[5 + i]);
        }

        p
      },
      media_width: u16::from_be_bytes(buf[14..16].try_into().unwrap()),
      medium_length: u16::from_be_bytes(buf[16..18].try_into().unwrap()),
      assigning_organization: match String::from_utf8(buf[20..28].to_vec()) {
        Ok(s) => { s },
        Err(_) => { "".to_string() },
      },
      medium_type_name: match String::from_utf8(buf[28..36].to_vec()) {
        Ok(s) => { s },
        Err(_) => { "".to_string() },
      },
      description: match String::from_utf8(buf[36..56].to_vec()) {
        Ok(s) => { s },
        Err(_) => { "".to_string() },
      },
    })
  }

  pub fn parse_buffer(&self, buf: &[u8]) -> Option <Vec <ReportDensitySupportOutput>> {
    if buf.len() < 4 {
      return None;
    }

    let len = std::cmp::min(buf.len(), 
        u16::from_be_bytes(buf[0..2].try_into().unwrap()) as usize);

    let mut rv: Vec <ReportDensitySupportOutput> = Vec::new();
    if self.medium_type {
      for i in (4..len).step_by(Self::MEDIUM_SIZE) {
        match self.parse_medium(&buf[i..(i + Self::MEDIUM_SIZE)]) {
          Some(thing) => { rv.push(ReportDensitySupportOutput::Medium(thing)); },
          None => { },
        }
      }
    } else {
      for i in (4..len).step_by(Self::DENSITY_SIZE) {
        match self.parse_density(&buf[i..(i + Self::DENSITY_SIZE)]) {
          Some(thing) => { rv.push(ReportDensitySupportOutput::Density(thing)); },
          None => { },
        }
      }
    }

    Some(rv)
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
