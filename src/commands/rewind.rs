use crate::sense::Sense;

/// SSC-4 Section 7.10
#[repr(C)]
#[derive(Default, Debug)]
pub struct Rewind {
  pub immed: bool,
}

impl Rewind {
  pub fn new() -> Rewind {
    Default::default()
  }
}

impl crate::NoIO for Rewind {
}

impl crate::Command for Rewind {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    data[0] = 0x01;
    data[1] = if self.immed { 0x1 } else { 0x0 };

    Ok(data)
  }

  fn parse_sense(&self, _sense: &Sense) -> String {
    String::from("")
  }
}


// C Functions

use crate::device::Device;
use crate::device::DeviceStatus;

#[no_mangle]
pub extern fn device_issue_rewind(
    device: *mut Device, cdb: *const Rewind) -> *mut DeviceStatus {
  let device = unsafe { &mut *device };
  let cdb = unsafe { &*cdb };

  let result = device.issue_cmd(cdb);
  match result {
    Ok(rv) => {
      let boxed_rv: Box <DeviceStatus> = Box::new(rv);
      Box::into_raw(boxed_rv)
    },
    Err(e) => {
      eprintln!("device_issue_rewind(): {}", e);
      std::ptr::null_mut()
    },
  }
}
