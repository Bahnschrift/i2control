use std::error::Error;

use hidapi::HidDevice;

use super::message::{MessageBuilder, default_header};

const OPERATION_ID: u8 = 0x06;

pub fn set_timeout(mouse: &HidDevice, timeout: Option<u8>) -> Result<(), Box<dyn Error>> {
    let mb = MessageBuilder::new(OPERATION_ID, 1)
        .with_header(|_| default_header(0x06, 0x00)[..3].to_vec())
        .push(timeout.unwrap_or(0xFF)); // Timeout disabled is signalled by sending 0xFF

    mb.build()?.send(mouse)?;
    Ok(())
}
