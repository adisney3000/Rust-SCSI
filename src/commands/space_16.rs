use crate::sense::Sense;

/// SSC-4 Section 7.12
#[derive(Default, Debug)]
pub struct Space16Positioning {
  pub partition_number: u8,
  pub logical_object_identifier: u64,
}

#[derive(Default, Debug)]
pub struct Space16 {
  pub code: u8,
  pub count: u64,
  //pub parameter_length: u16,
  pub positioning_info: Option <Space16Positioning>,
}

impl Space16 {
  pub const LOGICAL_BLOCKS: u8 = 0x0;
  pub const FILEMARKS: u8 = 0x1;
  pub const END_OF_DATA: u8 = 0x3;
  const OP_CODE: u8 = 0x91;

  pub fn new() -> Space16 {
    Default::default()
  }
}

impl crate::NoIO for Space16 {
}

impl crate::Command for Space16 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data =
        if self.positioning_info.is_none() {
          vec![0; 16]
        } else {
          vec![0; 32]
        };

    if !(0..16).contains(&self.code) {
      return Err("Code must be in the range 0..16");
    }

    data[0] = Self::OP_CODE;
    data[1] = self.code;
    data[4..12].copy_from_slice(&self.count.to_be_bytes());
    if self.positioning_info.is_some() {
      data[12..14].copy_from_slice(&16_u16.to_be_bytes());
      
      let pos = self.positioning_info.as_ref().unwrap();
      data[19] = pos.partition_number;
      data[20..28].copy_from_slice(
          &pos.logical_object_identifier.to_be_bytes());
    }

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    "".to_string()
  }
}
