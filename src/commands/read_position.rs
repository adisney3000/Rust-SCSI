use crate::sense::Sense;
use std::convert::TryInto;
//use std::fmt;

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
  pub const SHORT_FORM_LEN: u16 = 20;
  pub const LONG_FORM_LEN: u16 = 32;
  pub const EXTENDED_FORM_LEN: u16 = 32;
  const OP_CODE: u8 = 0x34;
  
  pub fn new() -> ReadPosition {
    Default::default()
  }

  pub fn output_len(&self) -> u16 {
    match self.service_action {
      ReadPosition::SHORT_FORM_BLOCK | ReadPosition::SHORT_FORM_VENDOR => {
        ReadPosition::SHORT_FORM_LEN
      },
      ReadPosition::LONG_FORM => { ReadPosition::LONG_FORM_LEN },
      ReadPosition::EXTENDED_FORM => { ReadPosition::EXTENDED_FORM_LEN },
      _ => { 0 },
    }
  }

  pub fn parse_buffer(&self, buf: &[u8]) -> Option <ReadPositionOutput> {
    match self.service_action {
      ReadPosition::SHORT_FORM_BLOCK | ReadPosition::SHORT_FORM_VENDOR => {
        if buf.len() < ReadPosition::SHORT_FORM_LEN.into() {
          return None;
        }

        Some(ReadPositionOutput::ShortForm(ReadPositionOutputShortForm {
          bop: buf[0] & 0x80 == 0x80,
          eop: buf[0] & 0x40 == 0x40,
          locu: buf[0] & 0x20 == 0x20,
          bycu: buf[0] & 0x10 == 0x10,
          rsvd: buf[0] & 0x08 == 0x08,
          lolu: buf[0] & 0x04 == 0x04,
          bpew: buf[0] & 0x01 == 0x01,
          partition_number: buf[1],
          first_logical_object_location:
              u32::from_be_bytes(buf[4..8].try_into().unwrap()),
          last_logical_object_location:
              u32::from_be_bytes(buf[8..12].try_into().unwrap()),
          number_of_logical_objects_in_object_buffer:
              u32::from_be_bytes([0, buf[13], buf[14], buf[15]].try_into().unwrap()),
          number_of_bytes_in_object_buffer:
              u32::from_be_bytes(buf[16..20].try_into().unwrap()),
        }))
      },
      ReadPosition::LONG_FORM => {
        if buf.len() < ReadPosition::LONG_FORM_LEN.into() {
          return None;
        }

        Some(ReadPositionOutput::LongForm(ReadPositionOutputLongForm {
          bop: buf[0] & 0x80 == 0x80,
          eop: buf[0] & 0x40 == 0x40,
          mpu: buf[0] & 0x08 == 0x08,
          lonu: buf[0] & 0x04 == 0x04,
          rsvd: buf[0] & 0x02 == 0x02,
          bpew: buf[0] & 0x01 == 0x01,
          partition_number:
              u32::from_be_bytes(buf[4..8].try_into().unwrap()),
          logical_object_number:
              u64::from_be_bytes(buf[8..16].try_into().unwrap()),
          logical_file_identifier:
              u64::from_be_bytes(buf[16..24].try_into().unwrap()),
        }))
      },
      ReadPosition::EXTENDED_FORM => {
        if buf.len() < ReadPosition::EXTENDED_FORM_LEN.into() {
          return None;
        }

        Some(ReadPositionOutput::ExtendedForm(ReadPositionOutputExtendedForm {
          bop: buf[0] & 0x80 == 0x80,
          eop: buf[0] & 0x40 == 0x40,
          locu: buf[0] & 0x20 == 0x20,
          bycu: buf[0] & 0x10 == 0x10,
          rsvd: buf[0] & 0x08 == 0x08,
          lolu: buf[0] & 0x04 == 0x04,
          bpew: buf[0] & 0x01 == 0x01,
          partition_number: buf[1],
          number_of_logical_objects_in_object_buffer:
              u32::from_be_bytes([0, buf[5], buf[6], buf[7]].try_into().unwrap()),
          first_logical_object_location:
              u64::from_be_bytes(buf[8..16].try_into().unwrap()),
          last_logical_object_location:
              u64::from_be_bytes(buf[16..24].try_into().unwrap()),
          number_of_bytes_in_object_buffer:
              u64::from_be_bytes(buf[24..32].try_into().unwrap()),
        }))
      },
      _ => {
        None
      },
    }
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

#[derive(Debug)]
pub enum ReadPositionOutput {
  ShortForm(ReadPositionOutputShortForm),
  LongForm(ReadPositionOutputLongForm),
  ExtendedForm(ReadPositionOutputExtendedForm),
}

/*
impl fmt::Display for ReadPositionOutput {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ReadPositionOutput::ShortForm(t) => {
        write!(f, "{}", t)
      },
      ReadPositionOutput::LongForm(t) => {
        write!(f, "{}", t)
      },
      ReadPositionOutput::ExtendedForm(t) => {
        write!(f, "{}", t)
      }
    }
  }
}
*/

#[derive(Debug)]
pub struct ReadPositionOutputShortForm {
  pub bop: bool,
  pub eop: bool,
  pub locu: bool,
  pub bycu: bool,
  pub rsvd: bool,
  pub lolu: bool,
  pub bpew: bool,
  pub partition_number: u8,
  pub first_logical_object_location: u32,
  pub last_logical_object_location: u32,
  pub number_of_logical_objects_in_object_buffer: u32,
  pub number_of_bytes_in_object_buffer: u32,
}

/* TODO
impl fmt::Display for ReadPositionOutputShortForm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "")
  }
}
*/

#[derive(Debug)]
pub struct ReadPositionOutputLongForm {
  pub bop: bool,
  pub eop: bool,
  pub mpu: bool,
  pub lonu: bool,
  pub rsvd: bool,
  pub bpew: bool,
  pub partition_number: u32,
  pub logical_object_number: u64,
  pub logical_file_identifier: u64,
}

/* TODO
impl fmt::Display for ReadPositionOutputLongForm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "")
  }
}
*/

#[derive(Debug)]
pub struct ReadPositionOutputExtendedForm {
  pub bop: bool,
  pub eop: bool,
  pub locu: bool,
  pub bycu: bool,
  pub rsvd: bool,
  pub lolu: bool,
  pub bpew: bool,
  pub partition_number: u8,
  pub number_of_logical_objects_in_object_buffer: u32,
  pub first_logical_object_location: u64,
  pub last_logical_object_location: u64,
  pub number_of_bytes_in_object_buffer: u64,
}

/* TODO
impl fmt::Display for ReadPositionOutputExtendedForm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "")
  }
}
*/
