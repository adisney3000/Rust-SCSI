//#![warn(missing_docs)]

use getopts::Options;
use std::env;
use scsi::commands::{*};
use scsi::Command;

fn print_usage(error: &str, program: &str, opts: Options) {
  let brief = format!("{}\nUsage: {} [options]", error, program);
  print!("{}", opts.usage(&brief));
}

fn main() {
  let args: Vec <String> = env::args().collect();
  let program = args[0].clone();
  let mut opts = Options::new();
  opts.reqopt("d", "device", "path to scsi device (required)", "DEVICE");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m },
    Err(e) => { print_usage(&e.to_string(), &program, opts); return },
  };

  let device_name = matches.opt_str("device").unwrap();

  let mut device = scsi::Device::new();
  let res = device.open(&device_name);
  if let Err(s) = res {
    println!("Error opening device: {}", s);
    return;
  }

  let rewind_cmd: Rewind = Rewind {
    immed: false,
  };
  let result = device.issue_cmd(&rewind_cmd);
  if let Ok(result) = &result {
    println!("{}", result);
  } else {
    println!("{}", result.unwrap_err());
  }

  let read_cmd: Read6 = Read6 {
    transfer_length: 80,
    ..Default::default()
  };
  let mut buf: [u8; 80] = [0; 80];
  let result = device.issue_cmd_with_output(&read_cmd, &mut buf);
  if let Ok(result) = &result {
    println!("{}", result);
  } else {
    println!("{}", result.unwrap_err());
  }
  println!("Buffer: {:x?}\n", buf);


  buf = [0; 80];
  let result = device.issue_cmd_with_output(&read_cmd, &mut buf);
  if let Ok(result) = result.as_ref() {
    println!("{}", result);
  } else {
    println!("{}", result.as_ref().unwrap_err());
  }
  if let Some(sense) = result.unwrap().sense {
    println!("{}", read_cmd.parse_sense(&sense));
  }
  println!("Buffer: {:x?}", buf);
}
