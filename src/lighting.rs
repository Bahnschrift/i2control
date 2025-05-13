use std::error::Error;

use hidapi::HidDevice;
use palettes::{
    BREATHING_PALETTE, GLORIOUS_PALETTE, RAVE_PALETTE, SEAMLESS_BREATHING_PALETTE, TAIL_PALETTE,
    WAVE_PALETTE,
};

use crate::{
    cli::LightingMode,
    report::{Report, ReportBuilder},
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
    ($p:expr, $rb:ident $(, $c:expr)?) => {{
        $($rb = $rb.extend_contiguous(&$c.bytes())?;)?
        for c in $p {
            $rb = $rb.extend_contiguous(&c.bytes())?;
        }
    }};
}

pub fn set_lighting(
    mouse: &HidDevice,
    brightness: u8,
    rate: u8,
    mode: LightingMode,
) -> Result<(), Box<dyn Error>> {
    let mut rb = ReportBuilder::new_with_header(OPERATION_ID, 3, 6, |index| {
        let mut header = Report::default_header(0x02, index).to_vec();
        header.push(mode.mode_id());
        header
    })
    .push(rate)?
    .push(brightness)?
    .push(mode.num_colours())?
    .push(rate)?
    .push(brightness)?;

    match mode {
        LightingMode::Off => (),
        LightingMode::Glorious => palette!(GLORIOUS_PALETTE, rb),
        LightingMode::SeamlessBreathing => palette!(SEAMLESS_BREATHING_PALETTE, rb),
        LightingMode::Breathing { col } => palette!(BREATHING_PALETTE, rb, col),
        LightingMode::SingleColour { col } => rb = rb.extend_contiguous(&col.bytes())?,
        LightingMode::BreathingSingleColour { col } => rb = rb.extend_contiguous(&col.bytes())?,
        LightingMode::Tail => palette!(TAIL_PALETTE, rb),
        LightingMode::Rave { col } => palette!(RAVE_PALETTE, rb, col),
        LightingMode::Wave => palette!(WAVE_PALETTE, rb),
    }

    let reports = rb.build();
    for report in reports {
        report.send(mouse)?;
    }

    Ok(())
}
