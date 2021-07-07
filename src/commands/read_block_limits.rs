use crate::sense::Sense;
use core::convert::TryInto;

/// SSC-4 Section 7.6
#[derive(Default, Debug)]
pub struct ReadBlockLimits {
  pub mloi: bool,
}

#[derive(Debug)]
pub enum ReadBlockLimitsOutput {
  BlockLimits(ReadBlockLimitsOutputRange),
  MaximumLogicalObjectIdentifier(u64),
}

#[derive(Default, Debug)]
pub struct ReadBlockLimitsOutputRange {
  pub granularity: u8,
  pub maximum_block_length_limit: u32,
  pub minimum_block_length_limit: u16,
}

impl ReadBlockLimits {
  const OP_CODE: u8 = 0x05;
  pub const BLOCK_LIMITS_SIZE: usize = 6;
  pub const MLOI_SIZE: usize = 20;
  
  pub fn new() -> ReadBlockLimits {
    Default::default()
  }

  pub fn output_len(&self) -> usize {
    if self.mloi { Self::MLOI_SIZE } else { Self::BLOCK_LIMITS_SIZE }
  }

  pub fn parse_buffer(&self, buf: &[u8]) -> Option <ReadBlockLimitsOutput> {
    if self.mloi {
      if buf.len() < Self::MLOI_SIZE {
        None
      } else {
        let limit = u64::from_be_bytes(buf[12..20].try_into().unwrap());
        Some(ReadBlockLimitsOutput::MaximumLogicalObjectIdentifier(limit))
      }
    } else if buf.len() < Self::BLOCK_LIMITS_SIZE {
      None
    } else {
      let block_limits = ReadBlockLimitsOutputRange {
        granularity: buf[0] & 0x1F,
        maximum_block_length_limit:
            u32::from_be_bytes([0, buf[1], buf[2], buf[3]]),
        minimum_block_length_limit:
            u16::from_be_bytes(buf[4..6].try_into().unwrap()),
      };
      Some(ReadBlockLimitsOutput::BlockLimits(block_limits))
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
