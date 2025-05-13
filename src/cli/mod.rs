pub mod rgb;

use clap::{Parser, Subcommand};
use rgb::Rgb;

// TODO: Change this to automatically detect connected devices matching known VID/PIDs.
//       This may require some sort of udev rule generation, at least on linux targets.

/// Model I2 Wireless Vendor ID
const I2_WL_VID: u16 = 0x93A;
/// Model I2 Wireless 2.4GHz wireless mode Product ID
const I2_WL_PID: u16 = 0x821D;

/// CLI Arguments
#[derive(Debug, Parser)]
#[command(
    name = "i2control",
    about = "CLI replacement for Glorious Control for the Model I2 Wireless"
)]
pub struct Args {
    /// Vendor ID
    #[arg(default_value_t = I2_WL_VID)]
    pub vid: u16,

    /// Product ID
    #[arg(default_value_t = I2_WL_PID)]
    pub pid: u16,

    #[command(subcommand)]
    pub command: Commands,
}

/// Command types
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(short_flag = 'p')]
    #[command(about = "Set the polling rate")]
    PollingRate { polling_rate: u16 },

    #[command(subcommand, subcommand_help_heading = "RGB Modes")]
    #[command(name = "rgb", short_flag = 'l')]
    #[command(about = "Configure RGB lighting settings")]
    LightingMode(LightingMode),

    #[command(short_flag = 'b')]
    #[command(about = "Get the current battery percentage")]
    Battery {},

    #[command(short_flag = 'd')]
    #[command(about = "Configure DPI profiles")]
    Dpi {},
}

/// Lighting effects corresponding to the options in Glorious Core.
/// Variants with an RGB value require a specified custom colour.
/// Casting to `u8` gives the correct byte for HID reports.
#[repr(u8)]
#[derive(Debug, Subcommand)]
pub enum LightingMode {
    Off,

    Glorious,

    SeamlessBreathing,

    // TODO: Make help for rgb command show that these modes require a colour
    Breathing { col: Rgb },

    SingleColour { col: Rgb },

    BreathingSingleColour { col: Rgb },

    Tail,

    Rave { col: Rgb },

    Wave,
}
