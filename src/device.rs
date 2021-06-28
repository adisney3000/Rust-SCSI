extern crate libc;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::io;
use std::ptr;
use std::fmt;
use crate::scsi_sg;
use crate::sense::{*};
use crate::{Command, NoIO, Output, Input};

/// SAM-5 Section 5.3
#[repr(C)]
#[derive(Debug)]
pub enum Status {
  Good,
  CheckCondition,
  ConditionMet,
  Busy,
  ReservationConflict,
  TaskSetFull,
  ACAActive,
  TaskAborted,
  Unknown(u8),
}

impl Status {
  pub fn from_u8(val: u8) -> Status {
    match val {
      0x00 => { Status::Good },
      0x02 => { Status::CheckCondition },
      0x04 => { Status::ConditionMet },
      0x08 => { Status::Busy },
      0x18 => { Status::ReservationConflict },
      0x28 => { Status::TaskSetFull },
      0x30 => { Status::ACAActive },
      0x40 => { Status::TaskAborted },
      _ => { Status::Unknown(val) },
    }
  }
}

/// Designed from the Linux sg documentation
#[repr(C)]
#[derive(Debug)]
pub enum HostStatus {
  OK,
  NoConnect,
  BusBusy,
  TimeOut,
  BadTarget,
  Abort,
  Parity,
  Error,
  Reset,
  BadIntr,
  Passthrough,
  SoftError,
  ImmRetry,
  Requeue,
  Unknown(u16),
}

impl HostStatus {
  pub fn from_u16(val: u16) -> HostStatus {
    match val {
      0x00 => { HostStatus::OK },
      0x01 => { HostStatus::NoConnect },
      0x02 => { HostStatus::BusBusy },
      0x03 => { HostStatus::TimeOut },
      0x04 => { HostStatus::BadTarget },
      0x05 => { HostStatus::Abort },
      0x06 => { HostStatus::Parity },
      0x07 => { HostStatus::Error },
      0x08 => { HostStatus::Reset },
      0x09 => { HostStatus::BadIntr },
      0x0A => { HostStatus::Passthrough },
      0x0B => { HostStatus::SoftError },
      0x0C => { HostStatus::ImmRetry },
      0x0D => { HostStatus::Requeue },
      _ => { HostStatus::Unknown(val) },
    }
  }
}

#[repr(C)]
#[derive(Debug)]
/// SCSI driver status
pub enum DriverStatus {
  OK,
  Busy,
  Soft,
  Media,
  Error,
  Invalid,
  Timeout,
  Hard,
  Sense,
  Unknown(u16),
}

impl DriverStatus {
  pub fn from_u16(val: u16) -> DriverStatus {
    let tmp = val & 0x0F;
    match tmp {
      0x00 => { DriverStatus::OK },
      0x01 => { DriverStatus::Busy },
      0x02 => { DriverStatus::Soft },
      0x03 => { DriverStatus::Media },
      0x04 => { DriverStatus::Error },
      0x05 => { DriverStatus::Invalid },
      0x06 => { DriverStatus::Timeout },
      0x07 => { DriverStatus::Hard },
      0x08 => { DriverStatus::Sense },
      _ => { DriverStatus::Unknown(val) },
    }
  }
}

#[repr(C)]
#[derive(Debug)]
/// SCSI driver suggested action
pub enum DriverSuggest {
  Nothing,
  Retry,
  Abort,
  Remap,
  Die,
  Sense,
  Unknown(u16),
}

impl DriverSuggest {
  pub fn from_u16(val: u16) -> DriverSuggest {
    let tmp = val & 0xF0;
    match tmp {
      0x00 => { DriverSuggest::Nothing },
      0x10 => { DriverSuggest::Retry },
      0x20 => { DriverSuggest::Abort },
      0x30 => { DriverSuggest::Remap },
      0x40 => { DriverSuggest::Die },
      0x80 => { DriverSuggest::Sense },
      _ => { DriverSuggest::Unknown(val) },
    }
  }
}

#[derive(Debug)]
/// SCSI device status information
pub struct DeviceStatus {
  pub status: Status,
  pub host_status: HostStatus,
  pub driver_status: DriverStatus,
  pub driver_suggest: DriverSuggest,
  pub sense: Option <Sense>,
}

impl fmt::Display for DeviceStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Err(e) = write!(f, concat!(
        "Status:        {:?}\n",
        "HostStatus:    {:?}\n",
        "DriverStatus:  {:?}\n",
        "DriverSuggest: {:?}",
        ),
        self.status,
        self.host_status,
        self.driver_status,
        self.driver_suggest,
    ){
      return Err(e);
    }

    if let Some(sense) = &self.sense {
      write!(f, "\n== Sense ==\n{}", sense)
    } else {
      Ok(())
    }
  }
}

/// SCSI device handle
pub struct Device {
  sg_fd: Option <libc::c_int>,
}

impl Device {
  const DEFAULT_HEADER: scsi_sg::sg_io_hdr = scsi_sg::sg_io_hdr {
    interface_id: 'S' as i32,
    dxfer_direction: 0,
    cmd_len: 0,
    mx_sb_len: 0,
    iovec_count: 0,
    dxfer_len: 0,
    dxferp: ptr::null_mut(),
    cmdp: ptr::null_mut(),
    sbp: ptr::null_mut(),
    timeout: 0,
    flags: 0,
    pack_id: 0,
    usr_ptr: ptr::null_mut(),
    status: 0,
    masked_status: 0,
    msg_status: 0,
    sb_len_wr: 0,
    host_status: 0,
    driver_status: 0,
    resid: 0,
    duration: 0,
    info: 0,
  };


  //First pass, let's just get it working very simply.
  //Make it work in lock step
  pub fn new() -> Device {
    Device {
      sg_fd: None,
    }
  }

  pub fn open(&mut self, path: &str) -> Result <(), String> {
    let tmp = CString::new(path);
    if tmp.is_err() {
      return Err("Could not convert path to CString".to_string());
    }
    
    let tmp = tmp.unwrap();
    let fd = unsafe {
      libc::open(tmp.as_c_str().as_ptr(), libc::O_RDWR)
    };

    if fd == -1 {
      return Err(io::Error::last_os_error().to_string());
    }

    self.sg_fd = Some(fd);
    Ok(())
  }

  pub fn close(&mut self) {
    if let Some(fd) = self.sg_fd {
      unsafe {libc::close(fd);}
      self.sg_fd = None;
    }
  }

  fn issue_cmd_internal <T>(&self, cdb: &T, mut header: scsi_sg::sg_io_hdr) -> Result <DeviceStatus, String>
    where T: Command,
  {
    if self.sg_fd == None {
      return Err("No device is currently open".to_string());
    }

    let bytes = cdb.to_bytes();
    if bytes.is_err() {
      return Err(format!("Error converting CDB to bytes: {}", bytes.unwrap_err().to_string()));
    }
    let mut bytes = bytes.unwrap();

    let mut sense_buffer: [u8; 255] = [0; 255];
    header.cmd_len = bytes.len() as u8;
    header.cmdp = bytes.as_mut_ptr();
    header.sbp = sense_buffer.as_mut_ptr();
    header.mx_sb_len = 255;

    //Send to device
    unsafe {
      let _rc = libc::ioctl(self.sg_fd.unwrap(), scsi_sg::SG_IO.into(), &mut header);
    }

    Ok(DeviceStatus {
      status: Status::from_u8(header.status),
      host_status: HostStatus::from_u16(header.host_status),
      driver_status: DriverStatus::from_u16(header.driver_status),
      driver_suggest: DriverSuggest::from_u16(header.driver_status),
      sense: Sense::from_buf(&sense_buffer),
    })
  }

  pub fn issue_cmd <T>(&self, cdb: &T) -> Result <DeviceStatus, String>
    where T: Command + NoIO,
  {
    let mut header = Self::DEFAULT_HEADER;
    header.dxfer_direction = scsi_sg::SG_DXFER_NONE;

    self.issue_cmd_internal(cdb, header)
  }

  pub fn issue_cmd_with_input <T>
      (&self, cdb: &T, buffer: &[u8]) -> Result <DeviceStatus, String>
    where T: Command + Input,
  {
    let mut header = Self::DEFAULT_HEADER;
    header.dxfer_direction = scsi_sg::SG_DXFER_TO_DEV;
    header.dxfer_len = buffer.len() as u32;
    header.dxferp = buffer.as_ptr() as *const _ as *mut libc::c_void;

    self.issue_cmd_internal(cdb, header)
  }

  /// Issue a command that will return data into a buffer
  pub fn issue_cmd_with_output <T>
      (&self, cdb: &T, buffer: &mut[u8]) -> Result <DeviceStatus, String>
    where T: Command + Output,
  {
    let mut header = Self::DEFAULT_HEADER;
    header.dxfer_direction = scsi_sg::SG_DXFER_FROM_DEV;
    header.dxfer_len = buffer.len() as u32;
    header.dxferp = buffer.as_mut_ptr() as *mut _ as *mut libc::c_void;

    self.issue_cmd_internal(cdb, header)
  }
}

impl Drop for Device {
  fn drop(&mut self) {
    if let Some(fd) = self.sg_fd {
      unsafe { libc::close(fd) };
    }
  }
}

impl Default for Device {
  fn default() -> Self {
    Self::new()
  }
}


// C Functions

#[no_mangle]
pub extern fn device_status_free(device_status: *mut DeviceStatus) {
  let boxed: Box <DeviceStatus> = unsafe{ Box::from_raw(device_status) };
  drop(boxed);
}

#[no_mangle]
pub extern fn device_status_to_stdout(device_status: *const DeviceStatus) {
  let device_status = unsafe { &*device_status };
  println!("{}", device_status);
}

#[no_mangle]
pub extern fn device_close(device: *mut Device) {
  let device = unsafe { &mut *device };
  device.close();
}

#[no_mangle]
pub extern fn device_open(device: *mut Device, path: *const c_char) -> bool {
  let device = unsafe { &mut *device };
  let path = unsafe { CStr::from_ptr(path) };
  let path = path.to_str();
  if let Err(e) = path {
    eprintln!("device_open(): {}", e);
    return false;
  }

  let result = device.open(path.unwrap());
  match result {
    Ok(_) => { true },
    Err(e) => { eprintln!("device_open(): {}", e); false },
  }
}

#[no_mangle]
pub extern fn device_new() -> *mut Device {
  let boxed_device: Box <Device> = Box::new(Device::new());
  Box::into_raw(boxed_device)
}

#[no_mangle]
pub extern fn device_delete(device: *mut Device) {
  if device.is_null() {
    return;
  }

  let boxed_device: Box <Device> = unsafe {
    Box::from_raw(device)
  };

  drop(boxed_device);
}
