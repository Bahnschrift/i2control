mod dump;

use clap::Parser;
use std::error::Error;

use hidapi::HidApi;

const I2_VID: u16 = 1133;
const I2_PID: u16 = 2737;

#[derive(Parser)]
struct Args {
    #[arg(default_value_t = I2_VID)]
    vid: u16,

    #[arg(default_value_t = I2_PID)]
    pid: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let api = HidApi::new()?;
    let mouse = api.open(args.vid, args.pid)?;

    dbg!(mouse.get_device_info()?);

    // Maybe this would work on windows?
    mouse.send_feature_report(get_dump!("../data/rgb/glorious/1"))?;
    mouse.send_feature_report(get_dump!("../data/rgb/glorious/2"))?;
    mouse.send_feature_report(get_dump!("../data/rgb/glorious/3"))?;

    Ok(())
}
