//#![warn(missing_docs)]

/*!
Interactive prompt to issue SCSI commands to a device. This uses the library
included in this crate.
*/

use getopts::Options;
use std::env;
use scsi::commands;
use scsi::Command;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::convert::TryInto;

struct ShellCommand {
  func: fn(&mut scsi::Device, &Options, &[&str]),
  options: Options,
}

fn main() {
  //Parse the args on the command line
  let args: Vec <String> = env::args().collect();
  let program = args[0].clone();
  let mut opts = Options::new();
  opts.reqopt("d", "device", "path to scsi device (required)", "DEVICE");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m },
    Err(e) => { print_usage(&e.to_string(), &program, opts); return },
  };

  //Open the device
  let device_name = matches.opt_str("device").unwrap();
  let mut device = scsi::Device::new();
  let res = device.open(&device_name);
  if let Err(s) = res {
    eprintln!("Error opening device: {}", s);
    return;
  }

  //Build the commands map for the interactive console
  let mut commands: HashMap <&str, ShellCommand> = HashMap::new();
  fill_commands(&mut commands);

  //Now accept commands
  let stdin = std::io::stdin();
  loop {
    print!("scsi> ");
    std::io::stdout().flush().expect("Couldn't flush stdout!");

    let mut line = String::new();
    match stdin.read_line(&mut line) {
      Ok(0) => { break; },
      Err(e) => { eprintln!("{}", e); break; },
      _ => {},
    }

    let tokens: Vec <&str> = line.split_whitespace().collect();
    if tokens.is_empty() {
      //Do nothing
    } else if tokens[0].eq("quit") {
      break;
    } else if let Some(cmd) = commands.get(&tokens[0]) {
      (cmd.func)(&mut device, &cmd.options, &tokens[1..]);
    } else {
      println!("Available commands:");
      for key in commands.keys() {
        println!("  {}", key);
      }
    }
  }

  device.close();
}

fn print_usage(error: &str, program: &str, opts: Options) {
  let brief = format!("{}\nUsage: {} [options]", error, program);
  print!("{}", opts.usage(&brief));
}

fn fill_commands(commands: &mut HashMap <&str, ShellCommand>) {
  let mut options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("i", "immed", "immediate flag");
  commands.insert("rewind", ShellCommand { func:rewind, options });

  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("s", "sili", "suppress incorrect-length indicator flag");
  options.optflag("f", "fixed", "read fixed size blocks flag");
  options.reqopt("l", "transfer_length", "number of bytes/blocks to read", "<u32>");
  options.optopt("", "output_file", "file to write output; stdout otherwise", "<str>");
  commands.insert("read_6", ShellCommand { func:read_6, options });

  options = Options::new();
  options.optflag("?", "help", "");
  options.optopt("a", "allow_overwrite", "allow overwrite type", "<u8>");
  options.optopt("p", "partition", "partition id", "<u8>");
  options.optopt("l", "logical_object_identifier", "", "<u64>");
  commands.insert("allow_overwrite",
      ShellCommand { func:allow_overwrite, options });

  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("", "fcs", "first command in sequence flag");
  options.optflag("", "lcs", "last command in sequence flag");
  options.optflag("i", "immed", "immediate flag");
  options.optflag("", "long", "long flag");
  options.optopt("m", "method", "erase method", "<u8>");
  options.optflag("", "smd", "security metadata flag");
  options.optflag("", "vcm", "vendor-specific control metadata flag");
  options.optopt("p", "partition", "partition id", "<u8>");
  options.optopt("l", "logical_object_identifier", "", "<u64>");
  commands.insert("erase_16",
      ShellCommand { func:erase_16, options });

  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("i", "immed", "immediate flag");
  options.optflag("", "long", "long flag");
  options.optopt("m", "method", "erase method", "<u8>");
  options.optflag("", "smd", "security metadata flag");
  options.optflag("", "vcm", "vendor-specific control metadata flag");
  commands.insert("erase_6",
      ShellCommand { func:erase_6, options });

  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("", "show_input", "display the inputs given to device");
  options.optflag("v", "verify", "verify flag");
  options.optflag("i", "immed", "immediate flag");
  options.optopt("f", "format", "format field", "<u8>");
  options.optopt("l", "transfer_length", "transfer length of input buffer", "<u16>");
  options.optopt("", "input_file", "file containing binary input", "<str>");
  commands.insert("format_medium",
      ShellCommand { func:format_medium, options });

  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("f", "fixed", "fixed flag");
  options.optflag("", "show_input", "display the inputs given to device");
  options.reqopt("l", "transfer_length", "number of bytes/blocks to write", "<u32>");
  options.reqopt("", "input_file", "file containing binary input", "<str>");
  commands.insert("write_6",
      ShellCommand { func:write_6, options });

  //options = Options::new();
  //options.optflag("?", "help", "");
}


//Macros to help write command functions

macro_rules! command_parse_args_or_return {
  ( $name:expr, $opts:expr, $args:expr ) => {
    match $opts.parse($args) {
      Ok(m) => {
        if !m.free.is_empty() || m.opt_present("help") {
          println!("{}{}", $opts.short_usage($name), $opts.usage(""));
          return
        } else { m }
      },
      Err(e) => { eprintln!("{}\n{}{}", e, $opts.short_usage($name), $opts.usage("")); return },
    }
  }
}

macro_rules! get_opt_or_return {
  ( $name:expr, $matches:expr ) => {
    match $matches.opt_get($name) {
      Ok(m) => { m.unwrap_or_default() },
      Err(e) => { eprintln!("Invalid {}: {}", $name, e); return },
    }
  };
}

macro_rules! print_status_or_error_and_return {
  ( $res:expr, $cmd:expr ) => {
    match $res {
      Ok(status) => {
        println!("{}", status);
        if let Some(sense) = status.sense {
          println!("{}", $cmd.parse_sense(&sense));
        }
      },
      Err(e) => {
        eprintln!("Failed: {}", e);
        return;
      },
    }
  }
}


//Command functions

fn rewind(device: &mut scsi::Device, opts: &Options, args: &[&str]) {
  let matches = command_parse_args_or_return!("rewind", opts, args);

  let cmd = commands::Rewind {
    immed: matches.opt_present("immed"),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_error_and_return!(result, cmd);
}

fn read_6(device: &mut scsi::Device, opts: &Options, args: &[&str]) {
  let matches = command_parse_args_or_return!("read_6", opts, args);

  let cmd = commands::Read6 {
    sili: matches.opt_present("sili"),
    fixed: matches.opt_present("fixed"),
    transfer_length: get_opt_or_return!("transfer_length", matches), 
  };

  let mut buf: Vec <u8> = Vec::new();
  buf.resize(cmd.transfer_length as usize, 0);
  
  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd_with_output(&cmd, buf.as_mut_slice());
  print_status_or_error_and_return!(result, cmd);

  //BLAH! Rewrite this to be more flat!
  match matches.opt_str("output_file") {
    Some(output_file_name) => {
      match File::create(&output_file_name) {
        Ok(mut f) => {
          match f.write(&buf) {
            Ok(l) => {
              if l < buf.len() {
                eprintln!("only wrote {} bytes to file", l);
              }
            },
            Err(e) => { eprintln!("failed to write buffer to file: {}", e); },
          }
        },
        Err(e) => {
          eprintln!("failed to create {}: {}", output_file_name, e);
        },
      }
    },
    None => {
      println!("Buffer: {:x?}", buf);
    },
  }
}

fn allow_overwrite(device: &mut scsi::Device, opts: &Options, args: &[&str]) {
  let matches = command_parse_args_or_return!("allow_overwrite", opts, args);

  let cmd = commands::AllowOverwrite {
    allow_overwrite: get_opt_or_return!("allow_overwrite", matches),
    partition: get_opt_or_return!("partition", matches),
    logical_object_identifier: get_opt_or_return!("logical_object_identifier", matches),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_error_and_return!(result, cmd);
}

fn erase_16(device: &mut scsi::Device, opts: &Options, args: &[&str]) {
  let matches = command_parse_args_or_return!("erase_16", opts, args);

  let cmd = commands::Erase16 {
    fcs: matches.opt_present("fcs"),
    lcs: matches.opt_present("lcs"),
    immed: matches.opt_present("immed"),
    long: matches.opt_present("long"),
    method: get_opt_or_return!("method", matches),
    smd: matches.opt_present("smd"),
    vcm: matches.opt_present("vcm"),
    partition: get_opt_or_return!("partition", matches),
    logical_object_identifier: get_opt_or_return!("logical_object_identifier", matches),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_error_and_return!(result, cmd);
}

fn erase_6(device: &mut scsi::Device, opts: &Options, args: &[&str]) {
  let matches = command_parse_args_or_return!("erase_6", opts, args);

  let cmd = commands::Erase6 {
    immed: matches.opt_present("immed"),
    long: matches.opt_present("long"),
    method: get_opt_or_return!("method", matches),
    smd: matches.opt_present("smd"),
    vcm: matches.opt_present("vcm"),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_error_and_return!(result, cmd);
}

fn format_medium(device: &mut scsi::Device, opts: &Options, args: &[&str]) {
  let matches = command_parse_args_or_return!("format_medium", opts, args);

  let cmd = commands::FormatMedium {
    verify: matches.opt_present("verify"),
    immed: matches.opt_present("immed"),
    format: get_opt_or_return!("format", matches),
    transfer_length: get_opt_or_return!("transfer_length", matches),
  };

  let mut buf: Vec <u8> = Vec::new();
  if cmd.transfer_length != 0 {
    let input_file_name = match matches.opt_str("input_file") {
      Some(s) => { s },
      None => {
        eprintln!("input_file is required when transfer_length is non-zero");
        return;
      },
    };

    let file = File::open(&input_file_name);
    if let Err(e) = file {
      eprintln!("failed to open {}: {}", input_file_name, e);
      return;
    }

    buf.resize(cmd.transfer_length.into(), 0);
    match file.unwrap().read(&mut buf) {
      Ok(read_bytes) => {
        if read_bytes != buf.len() {
          eprintln!("failed to read {} bytes from {}",
              cmd.transfer_length, input_file_name);
          return;
        }
      },
      Err(e) => {
        eprintln!("error occured while reading input file: {}", e);
        return;
      },
    }
  }

  println!("Issuing: {:#?}", cmd);
  if matches.opt_present("show_input") {
    println!("Input buffer: {:x?}", buf);
  }
  let result = device.issue_cmd_with_input(&cmd, &buf);
  print_status_or_error_and_return!(result, cmd);
}

fn write_6(device: &mut scsi::Device, opts: &Options, args: &[&str]) {
  let matches = command_parse_args_or_return!("write_6", opts, args);

  let cmd = commands::Write6 {
    fixed: matches.opt_present("fixed"),
    transfer_length: get_opt_or_return!("transfer_length", matches),
  };

  //Allocate the buffer and fill in
  let buf_len = cmd.transfer_length.try_into();
  if buf_len.is_err() {
    eprintln!("failed to convert transfer_length to usize!");
    return;
  }
  let mut buf: Vec <u8> = vec![0; buf_len.unwrap()];

  let input_file_name: String = get_opt_or_return!("input_file", matches);
  let file = File::open(&input_file_name);
  if let Err(e) = file {
    eprintln!("failed to open {}: {}", input_file_name, e);
    return;
  }
  
  match file.unwrap().read(&mut buf) {
    Ok(read_bytes) => {
      if read_bytes != buf.len() {
        eprintln!("failed to read {} bytes from {}",
            cmd.transfer_length, input_file_name);
        return;
      }
    },
    Err(e) => {
      eprintln!("error occured while reading input file: {}", e);
      return;
    },
  }

  //Now issue the command
  println!("Issuing: {:#?}", cmd);
  if matches.opt_present("show_input") {
    println!("Input buffer: {:x?}", buf);
  }
  let result = device.issue_cmd_with_input(&cmd, &buf);
  print_status_or_error_and_return!(result, cmd);
}
