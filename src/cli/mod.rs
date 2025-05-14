pub mod rgb;

use clap::{Parser, Subcommand, value_parser};
use rgb::Rgb;

// TODO: Change this to automatically detect connected devices matching known VID/PIDs.
//       This may require some sort of udev rule generation, at least on linux targets.

/// Model I2 Wireless Vendor ID
pub const I2_WL_VID: u16 = 0x93A;
/// Model I2 Wireless 2.4GHz wireless mode Product ID
pub const I2_WL_PID: u16 = 0x821D;

macro_rules! range {
    ($t:ty, $l:expr, $h:expr) => {
        value_parser!($t).range($l..=$h)
    };
}

/// CLI Arguments
#[derive(Debug, Parser)]
#[command(
    name = "i2control",
    about = "CLI replacement for Glorious Control for the Model I2 Wireless"
)]
pub struct Cli {
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
    /// Configure RGB lighting settings
    #[command(name = "rgb", short_flag = 'l')]
    Lighting {
        /// Ranges from 0 to 20 (inclusive)
        #[arg(value_parser = range!(u8, 0x00, 0x14))]
        brightness: u8,

        /// Ranges from 1 to 20 (inclusive)
        #[arg(value_parser = range!(u8, 0x01, 0x14))]
        rate: u8,

        #[command(subcommand)]
        mode: LightingMode,
    },

    /// Get the current battery percentage
    #[command(short_flag = 'b')]
    Battery {},

    /// Configure DPI profiles.
    ///
    /// Also allows configuring lift off distance, debouce time, and polling rate.
    ///
    /// Note that all of these will be overriden with defaults when using this command, unless
    /// otherwise specified.
    // TODO: Allow selecting a specific profile instead of just the first one
    #[command(short_flag = 'd')]
    Dpi {
        /// Lift off distance (mm).
        ///
        /// The maximum distance from a surface at which the mouse will register movement.
        ///
        /// Only values of 1 or 2 are accepted.
        #[arg(short = 'l', long = "lift")]
        #[arg(default_value_t = 0x01, value_parser = range!(u8, 0x01, 0x2))]
        lift_off_distance: u8,

        /// Debounce time (ms).
        ///
        /// Must be between 0 and 16 (inclusive).
        /// Odd values will be rounded up.
        #[arg(short = 'd', long = "debounce")]
        #[arg(default_value_t = 0x02, value_parser = range!(u8, 0x00, 0x10))]
        debounce_time: u8,

        /// Polling rate (Hz).
        ///
        /// Maximum value of 1000Hz, and will be rounded to the nearest of 125Hz, 250Hz, 500Hz, or
        /// 1000Hz.
        #[arg(short = 'p', long = "polling")]
        #[arg(default_value_t = 1000, value_parser = range!(u16, 0, 1000))]
        polling_rate: u16,

        /// DPI Stages.
        ///
        /// Each stage has a maximum value of 26000, and will be rounded to the nearest multiple of
        /// 50.
        #[arg(required = true, num_args = 1..=6, value_parser = range!(u16, 50, 26000))]
        dpi_stages: Vec<u16>,
    },

    /// Set the global inactivity timeout.
    #[command(short_flag = 't')]
    #[group(required = true, multiple = false)]
    Timeout {
        /// Disable global inactivity timeout
        #[arg(short = 'd', long = "disable")]
        disable: bool,

        /// Minutes of inactivity before sleep (0 to 100 inclusive).
        ///
        /// A value of 100 is rendered as infinity in Glorious Core.
        /// It is untested whether this is equivalent to disabling the timeout.
        #[arg(value_parser = range!(u8, 0x00, 0x64))]
        minutes: Option<u8>,
    },
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
