use clap::Subcommand;

use super::rgb::Rgb;

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
