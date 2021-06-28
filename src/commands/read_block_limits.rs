use crate::sense::Sense;
use core::convert::TryInto;

/// SSC-4 Section 7.6
#[derive(Default, Debug)]
pub struct ReadBlockLimits {
  pub mloi: bool,
}

pub struct ReadBlockLimitsRange {
  pub granularity: u8,
  pub maximum_block_length_limit: u32,
  pub minimum_block_length_limit: u16,
}

pub enum ReadBlockLimitsOutput {
  BlockLimits(ReadBlockLimitsRange),
  MaximumLogicalObjectIdentifier(u64),
}

impl ReadBlockLimits {
  const OP_CODE: u8 = 0x05;
  pub const BLOCK_LIMITS_BUF_SIZE: usize = 6;
  pub const MLOI_BUF_SIZE: usize = 20;
  
  pub fn new() -> ReadBlockLimits {
    Default::default()
  }

  pub fn parse_output(&self, buf: &[u8]) -> Result <ReadBlockLimitsOutput, &'static str> {
    if self.mloi {
      if buf.len() < Self::MLOI_BUF_SIZE {
        Err("Buffer length invalid")
      } else {
        let limit = u64::from_be_bytes(buf[12..20].try_into().unwrap());
        Ok(ReadBlockLimitsOutput::MaximumLogicalObjectIdentifier(limit))
      }
    } else if buf.len() < Self::BLOCK_LIMITS_BUF_SIZE {
      Err("Buffer length invalid")
    } else {
      let block_limits = ReadBlockLimitsRange {
        granularity: buf[0] & 0x1F,
        maximum_block_length_limit:
            u32::from_be_bytes([0, buf[1], buf[2], buf[3]]),
        minimum_block_length_limit:
            u16::from_be_bytes(buf[4..6].try_into().unwrap()),
      };
      Ok(ReadBlockLimitsOutput::BlockLimits(block_limits))
    }
  }
}

impl crate::Output for ReadBlockLimits {
}

impl crate::Command for ReadBlockLimits {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    data[0] = Self::OP_CODE;
    data[1] = if self.mloi { 0x1 } else { 0x0 };

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
