//#![warn(missing_docs)]

/*!
Interactive prompt to issue SCSI commands to a device. This uses the library
included in this crate.
*/

use getopts::{Options, Matches};
use std::env;
use scsi::commands;
use scsi::Command;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Write};
use std::convert::TryInto;

struct ShellCommand {
  func: fn(&mut scsi::Device, &Matches),
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
  let mut commands: BTreeMap <&str, ShellCommand> = BTreeMap::new();
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
      let opts = &cmd.options;
      match opts.parse(&tokens[1..]) {
        Ok(m) => {
          if !m.free.is_empty() || m.opt_present("help") {
            println!("{}{}", cmd.options.short_usage(&tokens[0]), cmd.options.usage(""));
            continue;
          }

          (cmd.func)(&mut device, &m);
        },
        Err(e) => {
          eprintln!("{}\n{}{}", e,
              opts.short_usage(&tokens[0]), opts.usage(""));
        },
      }
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

fn fill_commands(commands: &mut BTreeMap <&str, ShellCommand>) {
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

  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("i", "immed", "immediate flag");
  options.optflag("h", "hold", "hold flag");
  options.optflag("e", "eot", "end of tape flag");
  options.optflag("r", "reten", "retension flag");
  options.optflag("l", "load", "load flag");
  commands.insert("load_unload",
      ShellCommand { func:load_unload, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("", "bt", "TODO");
  options.optflag("", "cp", "TODO");
  options.optflag("i", "immed", "immediate flag");
  options.reqopt("l", "logical_object_identifier", "logical object identifier TODO", "<u32>");
  options.optopt("p", "partition", "partition", "<u8>");
  commands.insert("locate_10",
      ShellCommand { func:locate_10, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optopt("", "dest_type", "TODO", "<u8>"); //reqopt?? TODO
  options.optflag("", "rsvd", "TODO");
  options.optflag("", "cp", "TODO");
  options.optflag("i", "immed", "immediate flag");
  options.optflag("", "bam", "TODO");
  options.optopt("p", "partition", "partition", "<u8>");
  options.reqopt("l", "logical_object_identifier", "logical object identifier TODO", "<u32>");
  commands.insert("locate_16",
      ShellCommand { func:locate_16, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.reqopt("p", "prevent", "prevent", "<u8>");
  commands.insert("prevent_allow_medium_removal",
      ShellCommand { func:prevent_allow_medium_removal, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("s", "sili", "suppress incorrect-length indicator flag");
  options.optflag("f", "fixed", "read fixed size blocks flag");
  options.optopt("p", "partition", "partition", "<u8>");
  options.reqopt("", "logical_object_identifier", "logical object identifier TODO", "<u32>");
  options.reqopt("l", "transfer_length", "number of bytes/blocks to read", "<u32>");
  options.optopt("", "output_file", "file to write output; stdout otherwise", "<str>");
  commands.insert("read_16",
      ShellCommand { func:read_16, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("", "mloi", "maximum logical object identifier flag");
  commands.insert("read_block_limits",
      ShellCommand { func:read_block_limits, options });

  options = Options::new();
  options.optflag("?", "help", "");
  options.reqopt("a", "service_action", concat!(
      "determines type of read position response\n",
      "SHORT_FORM_BLOCK: 0\n",
      "SHORT_FORM_VENDOR: 1\n",
      "LONG_FORM: 6\n",
      "EXTENDED_FORM: 8"),
      "<u8>");
  options.optopt("l", "allocation_length", "length of output buffer for the response", "<u16>");
  commands.insert("read_position",
      ShellCommand { func:read_position, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("b", "bytord", "byte order flag");
  options.optflag("s", "sili", "suppress incorrect-length indicator flag");
  options.optflag("f", "fixed", "read fixed size blocks flag");
  options.optopt("p", "partition", "partition", "<u8>");
  options.reqopt("", "logical_object_identifier", "logical object identifier TODO", "<u32>");
  options.reqopt("l", "transfer_length", "number of bytes/blocks to read", "<u32>");
  options.optopt("", "output_file", "file to write output; stdout otherwise", "<str>");
  commands.insert("read_reverse_16",
      ShellCommand { func:read_reverse_16, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("b", "bytord", "byte order flag");
  options.optflag("s", "sili", "suppress incorrect-length indicator flag");
  options.optflag("f", "fixed", "read fixed size blocks flag");
  options.reqopt("l", "transfer_length", "number of bytes/blocks to read", "<u32>");
  options.optopt("", "output_file", "file to write output; stdout otherwise", "<str>");
  commands.insert("read_reverse_6",
      ShellCommand { func:read_reverse_6, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("s", "sili", "suppress incorrect-length indicator flag");
  options.optflag("", "rsvd", "TODO");
  options.optflag("f", "fixed", "read fixed size blocks flag");
  options.optopt("p", "partition", "partition", "<u8>");
  options.reqopt("", "logical_object_identifier", "logical object identifier TODO", "<u32>");
  options.reqopt("l", "transfer_length", "number of bytes/blocks to read", "<u32>");
  commands.insert("recover_buffered_data",
      ShellCommand { func:recover_buffered_data, options });

  //TODO Needs an output parser
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("", "medium_type", "medium type flag");
  options.optflag("", "media", "media flag");
  commands.insert("report_density_support",
      ShellCommand { func:report_density_support, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("i", "immed", "immediate flag");
  options.reqopt("%", "medium_for_proportion_value", "sets % space usable to this over 65535", "<u16>");
  commands.insert("set_capacity",
      ShellCommand { func:set_capacity, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.reqopt("", "code", "determines type of object to space over", "<u8>");
  options.reqopt("", "count", "number of objects to space over", "<u64>");
  options.optflag("", "position", "if set, position based on partition and logical_object_identifier before spacing");
  options.optopt("p", "partition_number", "partition", "<u8>");
  options.optopt("", "logical_object_identifier", "logical object identifier TODO", "<u64>");
  commands.insert("space_16",
      ShellCommand { func:space_16, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.reqopt("", "code", "determines type of object to space over", "<u8>");
  options.reqopt("", "count", "number of objects to space over", "<u32>");
  commands.insert("space_6",
      ShellCommand { func:space_6, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("", "vte", "verify to end of data flag");
  options.optflag("", "vlbpm", "verify logical block protection method flag");
  options.optflag("", "vbf", "verify by filemarks flag");
  options.optflag("i", "immed", "immediate flag");
  options.optflag("", "bytcmp", "byte compare flag");
  options.optflag("f", "fixed", "read fixed size blocks flag");
  options.optopt("p", "partition", "partition", "<u8>");
  options.reqopt("", "logical_object_identifier", "logical object identifier TODO", "<u32>");
  options.reqopt("l", "verification_length", "number of bytes/blocks to verify", "<u32>");
  options.optflag("", "show_input", "display the inputs given to device");
  options.reqopt("", "input_file", "file containing binary input", "<str>");
  commands.insert("verify_16",
      ShellCommand { func:verify_16, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("", "vte", "verify to end of data flag");
  options.optflag("", "vlbpm", "verify logical block protection method flag");
  options.optflag("", "vbf", "verify by filemarks flag");
  options.optflag("i", "immed", "immediate flag");
  options.optflag("", "bytcmp", "byte compare flag");
  options.optflag("f", "fixed", "read fixed size blocks flag");
  options.reqopt("l", "verification_length", "number of bytes/blocks to verify", "<u32>");
  options.optflag("", "show_input", "display the inputs given to device");
  options.reqopt("", "input_file", "file containing binary input", "<str>");
  commands.insert("verify_6",
      ShellCommand { func:verify_6, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("", "fcs", "first command in sequence flag");
  options.optflag("", "lcs", "last command in sequence flag");
  options.optflag("f", "fixed", "read fixed size blocks flag");
  options.optopt("p", "partition", "partition", "<u8>");
  options.reqopt("", "logical_object_identifier", "logical object identifier TODO", "<u32>");
  options.reqopt("l", "transfer_length", "number of bytes/blocks to write", "<u32>");
  options.optflag("", "show_input", "display the inputs given to device");
  options.reqopt("", "input_file", "file containing binary input", "<str>");
  commands.insert("write_16",
      ShellCommand { func:write_16, options });

  //TODO
  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("", "fcs", "first command in sequence flag");
  options.optflag("", "lcs", "last command in sequence flag");
  options.optflag("i", "immed", "immediate flag");
  options.optopt("p", "partition", "partition", "<u8>");
  options.reqopt("", "logical_object_identifier", "logical object identifier TODO", "<u32>");
  options.reqopt("c", "filemark_count", "number of filemarks to write", "<u32>");
  commands.insert("write_filemarks_16",
      ShellCommand { func:write_filemarks_16, options });

  options = Options::new();
  options.optflag("?", "help", "");
  options.optflag("i", "immed", "immediate flag");
  options.reqopt("c", "filemark_count", "number of filemarks to write", "<u32>");
  commands.insert("write_filemarks_6",
      ShellCommand { func:write_filemarks_6, options });
}


//Macros to help write command functions

macro_rules! get_opt_or_return {
  ( $name:expr, $matches:expr ) => {
    match $matches.opt_get($name) {
      Ok(m) => { m.unwrap_or_default() },
      Err(e) => { eprintln!("Invalid {}: {}", $name, e); return },
    }
  };
}

macro_rules! print_status_or_return_error {
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

fn rewind(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Rewind {
    immed: matches.opt_present("immed"),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn read_6(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Read6 {
    sili: matches.opt_present("sili"),
    fixed: matches.opt_present("fixed"),
    transfer_length: get_opt_or_return!("transfer_length", matches), 
  };

  let mut buf: Vec <u8> = Vec::new();
  buf.resize(cmd.transfer_length as usize, 0);
  
  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd_with_output(&cmd, buf.as_mut_slice());
  print_status_or_return_error!(result, cmd);

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

fn allow_overwrite(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::AllowOverwrite {
    allow_overwrite: get_opt_or_return!("allow_overwrite", matches),
    partition: get_opt_or_return!("partition", matches),
    logical_object_identifier: get_opt_or_return!("logical_object_identifier", matches),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn erase_16(device: &mut scsi::Device, matches: &Matches) {
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
  print_status_or_return_error!(result, cmd);
}

fn erase_6(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Erase6 {
    immed: matches.opt_present("immed"),
    long: matches.opt_present("long"),
    method: get_opt_or_return!("method", matches),
    smd: matches.opt_present("smd"),
    vcm: matches.opt_present("vcm"),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn format_medium(device: &mut scsi::Device, matches: &Matches) {
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
  print_status_or_return_error!(result, cmd);
}

fn write_6(device: &mut scsi::Device, matches: &Matches) {
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
  print_status_or_return_error!(result, cmd);
}

fn load_unload(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::LoadUnload {
    immed: matches.opt_present("immed"),
    hold: matches.opt_present("hold"),
    eot: matches.opt_present("eot"),
    reten: matches.opt_present("reten"),
    load: matches.opt_present("load"),
  };
  
  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn locate_10(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Locate10 {
    bt: matches.opt_present("bt"),
    cp: matches.opt_present("cp"),
    immed: matches.opt_present("immed"),
    logical_object_identifier: get_opt_or_return!("logical_object_identifier", matches),
    partition: get_opt_or_return!("partition", matches),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn locate_16(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Locate16 {
    dest_type: get_opt_or_return!("dest_type", matches),
    rsvd: matches.opt_present("rsvd"),
    cp: matches.opt_present("cp"),
    immed: matches.opt_present("immed"),
    bam: matches.opt_present("bam"),
    partition: get_opt_or_return!("partition", matches),
    logical_object_identifier: get_opt_or_return!("logical_object_identifier", matches),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn prevent_allow_medium_removal(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::PreventAllowMediumRemoval {
    prevent: get_opt_or_return!("prevent", matches),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn read_16(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Read16 {
    sili: matches.opt_present("sili"),
    fixed: matches.opt_present("fixed"),
    partition: get_opt_or_return!("partition", matches),
    logical_object_identifier: get_opt_or_return!("logical_object_identifier", matches),
    transfer_length: get_opt_or_return!("transfer_length", matches),
  };

  let mut buf: Vec <u8> = vec![0; cmd.transfer_length as usize];
  
  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd_with_output(&cmd, buf.as_mut_slice());
  print_status_or_return_error!(result, cmd);

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

fn read_block_limits(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::ReadBlockLimits {
    mloi: matches.opt_present("mloi"),
  };

  let mut buf: Vec <u8> = vec![0; cmd.output_len()];

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd_with_output(&cmd, &mut buf);
  print_status_or_return_error!(result, cmd);

  match cmd.parse_buffer(&buf) {
    Some(t) => { println!("Known output:\n{:#?}", t); },
    None => { println!("Unknown output:\n{:x?}", buf); },
  }
}

fn read_position(device: &mut scsi::Device, matches: &Matches) {
  let mut cmd = commands::ReadPosition {
    service_action: get_opt_or_return!("service_action", matches),
    allocation_length: get_opt_or_return!("allocation_length", matches),
  };

  if cmd.allocation_length == 0 {
    cmd.allocation_length = cmd.output_len() as u16;
  }

  let mut buf: Vec <u8> = vec![0; cmd.allocation_length.into()];

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd_with_output(&cmd, &mut buf);
  print_status_or_return_error!(result, cmd);

  match cmd.parse_buffer(&buf) {
    Some(t) => { println!("Known output:\n{:#?}", t); },
    None => { println!("Unknown output:\n{:x?}", buf); },
  }
}

fn read_reverse_16(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::ReadReverse16 {
    bytord: matches.opt_present("bytord"),
    sili: matches.opt_present("sili"),
    fixed: matches.opt_present("fixed"),
    partition: get_opt_or_return!("partition", matches),
    logical_object_identifier: get_opt_or_return!("logical_object_identifier", matches),
    transfer_length: get_opt_or_return!("transfer_length", matches),
  };

  let mut buf: Vec <u8> = Vec::new();
  buf.resize(cmd.transfer_length as usize, 0);
  
  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd_with_output(&cmd, buf.as_mut_slice());
  print_status_or_return_error!(result, cmd);

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

fn read_reverse_6(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::ReadReverse6 {
    bytord: matches.opt_present("bytord"),
    sili: matches.opt_present("sili"),
    fixed: matches.opt_present("fixed"),
    transfer_length: get_opt_or_return!("transfer_length", matches),
  };
  
  let mut buf: Vec <u8> = Vec::new();
  buf.resize(cmd.transfer_length as usize, 0);
  
  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd_with_output(&cmd, buf.as_mut_slice());
  print_status_or_return_error!(result, cmd);

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

fn recover_buffered_data(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::RecoverBufferedData {
    sili: matches.opt_present("sili"),
    fixed: matches.opt_present("fixed"),
    transfer_length: get_opt_or_return!("transfer_length", matches),
  };

  let mut buf: Vec <u8> = Vec::new();
  buf.resize(cmd.transfer_length as usize, 0);
  
  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd_with_output(&cmd, buf.as_mut_slice());
  print_status_or_return_error!(result, cmd);

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

fn report_density_support(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::ReportDensitySupport {
    medium_type: matches.opt_present("medium_type"),
    media: matches.opt_present("media"),
    allocation_length: u16::MAX,
  };

  let mut buf: Vec <u8> = vec![0; cmd.allocation_length as usize];

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd_with_output(&cmd, buf.as_mut_slice());
  print_status_or_return_error!(result, cmd);
  
  match cmd.parse_buffer(&buf) {
    Some(t) => { println!("Known output:\n{:#?}", t); },
    None => { println!("Unknown output:\n{:x?}", buf); },
  }
}

fn set_capacity(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::SetCapacity {
    immed: matches.opt_present("immed"),
    medium_for_proportion_value:
        get_opt_or_return!("medium_for_proportion_value", matches),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn space_16(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Space16 {
    code: get_opt_or_return!("code", matches),
    count: get_opt_or_return!("count", matches),
    positioning_info: if matches.opt_present("position") {
      Some(commands::Space16Positioning {
        partition_number: get_opt_or_return!("partition_number", matches),
        logical_object_identifier: get_opt_or_return!("logical_object_identifier", matches),
      })
    } else {
      None
    },
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn space_6(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Space6 {
    code: get_opt_or_return!("code", matches),
    count: get_opt_or_return!("count", matches),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn verify_16(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Verify16 {
    vte: matches.opt_present("vte"),
    vlbpm: matches.opt_present("vlbpm"),
    vbf: matches.opt_present("vbf"),
    immed: matches.opt_present("immed"),
    bytcmp: matches.opt_present("bytcmp"),
    fixed: matches.opt_present("fixed"),
    partition: get_opt_or_return!("partition", matches),
    logical_object_identifier:
        get_opt_or_return!("logical_object_identifier", matches),
    verification_length: get_opt_or_return!("verification_length", matches),
  };

  //Allocate the buffer and fill in if bytcmp is set
  let buf_len = if cmd.bytcmp {
    cmd.verification_length.try_into()
  } else { Ok(0) };
  if buf_len.is_err() {
    eprintln!("failed to convert verification_length to usize!");
    return;
  }
  let mut buf: Vec <u8> = vec![0; buf_len.unwrap()];

  if buf.len() > 0 {
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
              cmd.verification_length, input_file_name);
          return;
        }
      },
        Err(e) => {
          eprintln!("error occured while reading input file: {}", e);
          return;
        },
    }
  }

  //Now issue the command
  println!("Issuing: {:#?}", cmd);
  if matches.opt_present("show_input") {
    println!("Input buffer: {:x?}", buf);
  }
  let result = device.issue_cmd_with_input(&cmd, &buf);
  print_status_or_return_error!(result, cmd);
}

fn verify_6(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Verify6 {
    vte: matches.opt_present("vte"),
    vlbpm: matches.opt_present("vlbpm"),
    vbf: matches.opt_present("vbf"),
    immed: matches.opt_present("immed"),
    bytcmp: matches.opt_present("bytcmp"),
    fixed: matches.opt_present("fixed"),
    verification_length: get_opt_or_return!("verification_length", matches),
  };

  //Allocate the buffer and fill in if bytcmp is set
  let buf_len = if cmd.bytcmp {
    cmd.verification_length.try_into()
  } else { Ok(0) };
  if buf_len.is_err() {
    eprintln!("failed to convert verification_length to usize!");
    return;
  }
  let mut buf: Vec <u8> = vec![0; buf_len.unwrap()];

  if buf.len() > 0 {
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
              cmd.verification_length, input_file_name);
          return;
        }
      },
        Err(e) => {
          eprintln!("error occured while reading input file: {}", e);
          return;
        },
    }
  }

  //Now issue the command
  println!("Issuing: {:#?}", cmd);
  if matches.opt_present("show_input") {
    println!("Input buffer: {:x?}", buf);
  }
  let result = device.issue_cmd_with_input(&cmd, &buf);
  print_status_or_return_error!(result, cmd);
}

fn write_16(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::Write16 {
    fcs: matches.opt_present("fcs"),
    lcs: matches.opt_present("lcs"),
    rsvd: matches.opt_present("rsvd"),
    fixed: matches.opt_present("fixed"),
    partition: get_opt_or_return!("partition", matches),
    logical_object_identifier:
        get_opt_or_return!("logical_object_identifier", matches),
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
  print_status_or_return_error!(result, cmd);
}

fn write_filemarks_16(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::WriteFilemarks16 {
    fcs: matches.opt_present("fcs"),
    lcs: matches.opt_present("lcs"),
    immed: matches.opt_present("immed"),
    partition: get_opt_or_return!("partition", matches),
    logical_object_identifier:
        get_opt_or_return!("logical_object_identifier", matches),
    filemark_count: get_opt_or_return!("filemark_count", matches),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}

fn write_filemarks_6(device: &mut scsi::Device, matches: &Matches) {
  let cmd = commands::WriteFilemarks6 {
    immed: matches.opt_present("immed"),
    filemark_count: get_opt_or_return!("filemark_count", matches),
  };

  println!("Issuing: {:#?}", cmd);
  let result = device.issue_cmd(&cmd);
  print_status_or_return_error!(result, cmd);
}
