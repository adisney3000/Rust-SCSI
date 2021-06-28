//#![warn(missing_docs)]

/*!
Abstraction layer for issuing commands through the Linux [`SG_IO`] interface.
Users will still need to understand the SCSI standard to know how to
setup Command Descriptor Blocks (CDB) and get the desired results.

# Usage

Generally, the major components of this library are:
* [`Device`](struct@Device)
* [`Command`](trait@Command)
* [`Sense`](struct@Sense)

First open a SCSI device with the Device structure.
```
let mut device = scsi::Device::new();
let result = device.open("/path/to/device");
```

From there simply create commands and send them to the device.
```
# use scsi::commands::Rewind;
# let mut device = scsi::Device::new();
# let result = device.open("/path/to/device");

let rewind_cmd: Rewind = Rewind {
  immed: false,
};

let result = device.issue_cmd(&rewind_cmd);
```

Finally, the result is a DeviceStatus which contains some Enums
for various status plus an optional Sense structure [^better_device_status].


[`SG_IO`]: https://www.kernel.org/doc/html/latest/scsi/scsi-generic.html

[^better_device_status]: (Test footnote) Possibly make this a bit more abstract
or add functions for understanding various combinations of status.
*/

/// SCSI Command structures implement this trait
pub trait Command {
  fn to_bytes(&self) -> Result <Vec <u8>, &'static str>;

  //Not sure if parse_sense needs to be part of this trait since not all commands
  //will have anything to parse but it does make it easier for the user to generically
  //just call parse_sense for every command issued.
  fn parse_sense(&self, sense: &Sense) -> String;
}

/// Command marker trait to indicate it has associated input to be sent to the SCSI device.
pub trait Input {
}

/// Command marker trait to indicate it has associated output to be received from the SCSI device.
pub trait Output {
}

/// Command marker trait to indicate it has no associated input or output.
pub trait NoIO {
}

pub mod commands {
  //SPC-3
  //mod inquiry;
  //mod log_select;
  //mod log_sense;
  //mod mode_select_6;
  //mod mode_select_10;
  //mod mode_sense_6;
  //mod mode_sense_10;

  //SSC-4
  mod allow_overwrite;
  pub use allow_overwrite::AllowOverwrite;

  mod erase_6;
  pub use erase_6::Erase6;

  mod erase_16;
  pub use erase_16::Erase16;

  mod format_medium;
  pub use format_medium::FormatMedium;

  mod load_unload;
  pub use load_unload::LoadUnload;

  mod locate_10;
  pub use locate_10::Locate10;

  mod locate_16;
  pub use locate_16::Locate16;

  mod prevent_allow_medium_removal;
  pub use prevent_allow_medium_removal::PreventAllowMediumRemoval;

  mod read_block_limits;
  pub use read_block_limits::ReadBlockLimits;
  pub use read_block_limits::ReadBlockLimitsOutput;
  pub use read_block_limits::ReadBlockLimitsRange;
    //TODO This requires output to be parsed. Thinking that in general
    //  if output is to be parsed that the user would have to parse it
    //  with a method on the SCSI command. So either
    //  static like ReadBlockLimits::parse()
    //  or method cmd.parse()
    //  Depends on if the specifics of the command are important.
    //  Sense might be in a similar boat.

  mod read_position;
  pub use read_position::ReadPosition;
    //TODO Also needs a parser

  mod read_reverse_6;
  pub use read_reverse_6::ReadReverse6;

  mod read_reverse_16;
  pub use read_reverse_16::ReadReverse16;

  mod read_6;
  pub use read_6::Read6;

  mod read_16;
  pub use read_16::Read16;

  mod recover_buffered_data;
  pub use recover_buffered_data::RecoverBufferedData;

  mod report_density_support;
  pub use report_density_support::ReportDensitySupport;
    //TODO Needs parser

  mod rewind;
  pub use rewind::Rewind;

  mod set_capacity;
  pub use set_capacity::SetCapacity;

  mod space_6;
  pub use space_6::Space6;

  mod space_16;
  pub use space_16::Space16;

  mod verify_6;
  pub use verify_6::Verify6;

  mod verify_16;
  pub use verify_16::Verify16;

  mod write_filemarks_6;
  pub use write_filemarks_6::WriteFilemarks6;

  mod write_filemarks_16;
  pub use write_filemarks_16::WriteFilemarks16;

  mod write_6;
  pub use write_6::Write6;

  mod write_16;
  pub use write_16::Write16;
}

/// cbindgen:ignore
mod scsi_sg;

mod device;
pub use device::Device;
pub use device::DeviceStatus;
pub use device::DriverSuggest;
pub use device::DriverStatus;
pub use device::Status;
pub use device::HostStatus;

mod sense;
pub use sense::Sense;
