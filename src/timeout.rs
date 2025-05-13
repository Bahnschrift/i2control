use std::error::Error;

use hidapi::HidDevice;

use crate::report::{Report, ReportBuilder};

const OPERATION_ID: u8 = 0x06;

pub fn set_timeout(mouse: &HidDevice, timeout: Option<u8>) -> Result<(), Box<dyn Error>> {
    let rb = ReportBuilder::new_with_header(OPERATION_ID, 1, 3, |_| {
        Report::default_header(0x06, 0x00)[..3].to_vec()
    })
    .push(timeout.unwrap_or(0xFF))?; // Timeout disabled is signalled by sending 0xFF

    let reports = rb.build();
    for report in reports {
        report.send(mouse)?;
    }

    Ok(())
}
