use std::error::Error;

use hidapi::HidDevice;

use crate::message::MessageBuilder;

const OPERATION_ID: u8 = 0x04;

const POLLING_RATES: [u16; 4] = [125, 250, 500, 1000];

fn polling_rate_id(polling_rate: u16) -> u8 {
    let (i, _) = POLLING_RATES
        .into_iter()
        .enumerate()
        .rev()
        .min_by_key(|(_, r)| polling_rate.abs_diff(*r))
        .unwrap();

    i as u8 + 1
}

pub fn set_dpi(
    mouse: &HidDevice,
    lift_off_distance: u8,
    debounce_time: u8,
    polling_rate: u16,
    mut stages: Vec<u16>,
) -> Result<(), Box<dyn Error>> {
    let debounce_time = debounce_time.div_ceil(2) * 2; // Round up to even integers
    let polling_rate = polling_rate_id(polling_rate);

    for s in stages.iter_mut() {
        *s = (*s + 25) / 50; // DPI must be a multiple of 50, so we divide and round
    }

    let mut mb = MessageBuilder::new(OPERATION_ID, 4)
        .push(0x00) // Select first DPI stage
        .push(stages.len() as u8)
        .push(lift_off_distance)
        .push(debounce_time)
        .push(polling_rate)
        .push(0x00);

    for stage in stages {
        mb = mb.push_block(&[
            (stage & 0x00FF) as u8,
            ((stage & 0xFF00) >> 8) as u8,
            0xFF,
            0xA4,
            0x0D,
        ]);
    }

    mb.build()?.send(mouse)?;
    Ok(())
}
