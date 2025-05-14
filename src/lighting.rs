use std::error::Error;

use hidapi::HidDevice;
use palettes::{
    BREATHING_PALETTE, GLORIOUS_PALETTE, RAVE_PALETTE, SEAMLESS_BREATHING_PALETTE, TAIL_PALETTE,
    WAVE_PALETTE,
};

use crate::{
    cli::LightingMode,
    message::{MessageBuilder, default_header},
};

// TODO: Move this to a LightingMode impl
mod palettes {
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

const OPERATION_ID: u8 = 0x02;

macro_rules! palette {
    ($p:expr, $mb:ident $(, $c:expr)?) => {{
        $($mb = $mb.push_block(&$c.bytes());)?
        for c in $p {
            $mb = $mb.push_block(&c.bytes());
        }
    }};
}

pub fn set_lighting(
    mouse: &HidDevice,
    brightness: u8,
    rate: u8,
    mode: LightingMode,
) -> Result<(), Box<dyn Error>> {
    let mut mb = MessageBuilder::new(OPERATION_ID, 3)
        .with_header(|i| {
            let mut header = default_header(0x02, i).to_vec();
            header.push(mode.mode_id());
            header
        })
        .push(rate)
        .push(brightness)
        .push(mode.num_colours())
        .push(rate)
        .push(brightness);

    match mode {
        LightingMode::Off => (),
        LightingMode::Glorious => palette!(GLORIOUS_PALETTE, mb),
        LightingMode::SeamlessBreathing => palette!(SEAMLESS_BREATHING_PALETTE, mb),
        LightingMode::Breathing { col } => palette!(BREATHING_PALETTE, mb, col),
        LightingMode::SingleColour { col } => mb = mb.push_block(&col.bytes()),
        LightingMode::BreathingSingleColour { col } => mb = mb.push_block(&col.bytes()),
        LightingMode::Tail => palette!(TAIL_PALETTE, mb),
        LightingMode::Rave { col } => palette!(RAVE_PALETTE, mb, col),
        LightingMode::Wave => palette!(WAVE_PALETTE, mb),
    }

    mb.build()?.send(mouse)?;
    Ok(())
}
