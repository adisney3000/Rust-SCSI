use crate::sense::Sense;

/// SSC-4 Section 6.4
#[repr(C)]
#[derive(Default, Debug)]
pub struct Read6 {
  pub sili: bool,
  pub fixed: bool,
  pub transfer_length: u32,
}

impl Read6 {
  const OP_CODE: u8 = 0x08;
  
  pub fn new() -> Read6 {
    Default::default()
  }
}

impl crate::Output for Read6 {
}

impl crate::Command for Read6 {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str> {
    let mut data = vec![0; 6];

    if !(0..(2_u32.pow(24))).contains(&self.transfer_length) {
      return Err("Transfer length must be in the range 0..2^24");
    }

    data[0] = Self::OP_CODE;
    data[1] =
        if self.sili { 0x2 } else { 0x0 } |
        if self.fixed { 0x1 } else { 0x0 };
    data[2..5].copy_from_slice(&self.transfer_length.to_be_bytes()[1..]);

    Ok(data)
  }

  //TODO Might be a better way to send this to a Formatter so it doesn't have to make a String.
  //  Can't just impl fmt because we need the Sense.
  //  One way might be to attach a Sense ref to the instance then fmt() could look at it.
  //  Remove it after fmt().
  fn parse_sense(&self, sense: &Sense) -> String {
    //TODO Check the conditions that should be set
    //  Also this is just a quick example. Might need to be i64?
    format!(concat!(
        "== Read6 Specific ==\n",
        "Requested transfer length minus the actual block length is {}"
        ),
        u64::from_be_bytes(sense.information)
    )
  }
}

#[test]
fn to_bytes_test() {
  let read_cmd = Read6 {
    sili: true,
    fixed: true,
    transfer_length: 300,
  };

  let buffer = read_cmd.to_bytes();
  assert_eq!(buffer, Ok(vec![0x08_u8, 0x03, 0x00, 0x01, 0x2C, 0x00]));
}

#[test]
fn transfer_length_test() {
  let read_cmd = Read6 {
    sili: true,
    fixed: true,
    transfer_length: 2_u32.pow(24),
  };

  let buffer = read_cmd.to_bytes();
  assert_eq!(buffer, Err("Transfer length must be in the range 0..2^24"));
}


// C Functions
use crate::device::Device;
use crate::device::DeviceStatus;

#[no_mangle]
pub extern fn device_issue_read6(
    device: *mut Device, cdb: *const Read6, buf: *mut u8, size: usize) -> *mut DeviceStatus {
  let device = unsafe { &mut *device };
  let cdb = unsafe { &*cdb };
  let mut buf = unsafe { std::slice::from_raw_parts_mut(buf, size) };

  let result = device.issue_cmd_with_output(cdb, &mut buf);
  match result {
    Ok(rv) => {
      let boxed_rv: Box <DeviceStatus> = Box::new(rv);
      Box::into_raw(boxed_rv)
    },
    Err(e) => {
      eprintln!("device_issue_read6: {}", e);
      std::ptr::null_mut()
    },
  }
}

use crate::Command;

#[no_mangle]
pub extern fn read_6_status_to_stdout(cmd: *const Read6, status: *mut DeviceStatus) {
  let cmd = unsafe { &*cmd };
  let status = unsafe { &mut *status };
  if let Some(sense) = &status.sense {
    println!("{}", cmd.parse_sense(sense));
  }
}
