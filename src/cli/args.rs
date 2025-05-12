use clap::{Parser, ValueEnum};

const I2_VID: u16 = 0x93A;
const I2_PID: u16 = 0x821D;

#[derive(Debug, Parser)]
pub struct Args {
    /// Vendor ID
    #[arg(default_value_t = I2_VID)]
    pub vid: u16,

    /// Product ID
    #[arg(default_value_t = I2_PID)]
    pub pid: u16,

    // #[arg(short, long)]
    // pub rgb: Option<RGBMode>,

    /// Polling rate (in Hz). Must be one of 125, 250, 500, or 1000.
    #[arg(short, long)]
    pub polling_rate: Option<u16>,
}
