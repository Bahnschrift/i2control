pub mod rgb;

use clap::{Parser, Subcommand, value_parser};
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

    // #[command(subcommand, subcommand_help_heading = "RGB Modes")]
    #[command(name = "rgb", short_flag = 'l')]
    #[command(about = "Configure RGB lighting settings")]
    Lighting {
        /// Ranges from 0 to 20 (inclusive)
        #[arg(value_parser = value_parser!(u8).range(0x00..=0x14))]
        brightness: u8,

        /// Ranges from 1 to 20 (inclusive)
        #[arg(value_parser = value_parser!(u8).range(0x01..=0x14))]
        rate: u8,

        #[command(subcommand)]
        mode: LightingMode,
    },

    #[command(short_flag = 'b')]
    #[command(about = "Get the current battery percentage")]
    Battery {},

    #[command(short_flag = 'd')]
    #[command(about = "Configure DPI profiles")]
    Dpi {},
}

/// Lighting effects corresponding to the options in Glorious Core.
/// Variants with an RGB value require a specified custom colour.
#[derive(Debug, Subcommand)]
pub enum LightingMode {
    Off,
    Glorious,
    SeamlessBreathing,
    // TODO: Make help for rgb command show that these modes require a colour.
    //       It would also be nice to work out how to make these fields anonymous.
    Breathing {
        col: Rgb,
    },
    #[command(alias = "single-color")]
    SingleColour {
        col: Rgb,
    },
    #[command(alias = "breathing-single-color")]
    BreathingSingleColour {
        col: Rgb,
    },
    Tail,
    Rave {
        col: Rgb,
    },
    Wave,
}

impl LightingMode {
    /// Returns the associated ID for HID reports
    pub fn mode_id(&self) -> u8 {
        match self {
            LightingMode::Off => 0x00,
            LightingMode::Glorious => 0x01,
            LightingMode::SeamlessBreathing => 0x02,
            LightingMode::Breathing { col: _ } => 0x03,
            LightingMode::SingleColour { col: _ } => 0x04,
            LightingMode::BreathingSingleColour { col: _ } => 0x05,
            LightingMode::Tail => 0x06,
            LightingMode::Rave { col: _ } => 0x07,
            LightingMode::Wave => 0x08,
        }
    }

    /// Returns the number of colours required by the given lighting mode
    pub fn num_colours(&self) -> u8 {
        match self {
            LightingMode::Off => 0x00,
            LightingMode::Glorious => 0x07,
            LightingMode::SeamlessBreathing => 0x07,
            LightingMode::Breathing { col: _ } => 0x06,
            LightingMode::SingleColour { col: _ } => 0x01,
            LightingMode::BreathingSingleColour { col: _ } => 0x01,
            LightingMode::Tail => 0x06,
            LightingMode::Rave { col: _ } => 0x02,
            LightingMode::Wave => 0x07,
        }
    }
}
