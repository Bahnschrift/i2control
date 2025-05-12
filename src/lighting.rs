use crate::cli::rgb::Rgb;

pub mod palettes {
    use crate::{cli::rgb::Rgb, rgb};

    pub const GLORIOUS_PALETTE: [Rgb; 7] = [
        rgb!(0xFF0000),
        rgb!(0xFFC400),
        rgb!(0xFBFF00),
        rgb!(0x00FF33),
        rgb!(0x00FBFF),
        rgb!(0x0004FF),
        rgb!(0xE600FF),
    ];

    pub const SEAMLESS_BREATHING_PALETTE: [Rgb; 7] = GLORIOUS_PALETTE;

    /// Also requires one custom colour at the start
    pub const BREATHING_PALETTE: [Rgb; 5] = [
        rgb!(0x0AFF7D),
        rgb!(0xFF600A),
        rgb!(0x0AFFE5),
        rgb!(0xFF0AD4),
        rgb!(0xFF0000),
    ];

    pub const TAIL_PALETTE: [Rgb; 6] = [
        rgb!(0xFFF60A),
        rgb!(0x0AFF7D),
        rgb!(0xFF600A),
        rgb!(0x0AFFE5),
        rgb!(0xFF0AD4),
        rgb!(0xFF0000),
    ];

    /// Also requires one custom colour at the start
    pub const RAVE_PALETTE: [Rgb; 1] = [rgb!(0xFFFF00)];

    pub const WAVE_PALETTE: [Rgb; 7] = [
        rgb!(0xFFF60A),
        rgb!(0x0AFF7D),
        rgb!(0xFF600A),
        rgb!(0x0AFFE5),
        rgb!(0xFF0AD4),
        rgb!(0x000000),
        rgb!(0xFF0000),
    ];
}

/// Lighting effects corresponding to the options in Glorious Core.
/// Variants with an RGB value require a specified custom colour.
/// Casting to `u8` gives the correct byte for HID reports.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RGBMode {
    Off,
    Glorious,
    SeamlessBreathing,
    Breathing(Rgb),
    SingleColour(Rgb),
    BreathingSingleColour(Rgb),
    Tail,
    Rave(Rgb),
    Wave,
}
