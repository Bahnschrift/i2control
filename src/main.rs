mod dump;

use clap::{Parser, ValueEnum};
use std::error::Error;

use hidapi::{HidApi, HidDevice};

const I2_VID: u16 = 2362;
const I2_PID: u16 = 33309;

#[derive(Parser)]
struct Args {
    #[arg(default_value_t = I2_VID)]
    vid: u16,

    #[arg(default_value_t = I2_PID)]
    pid: u16,

    #[arg(short, long)]
    rgb: Option<RGBMode>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum RGBMode {
    Off,
    Glorious,
    Breathing,
}

fn set_rgb(mouse: &HidDevice, mode: RGBMode) -> Result<(), Box<dyn Error>> {
    match mode {
        RGBMode::Off => {
            mouse.send_feature_report(get_dump!("../data/rgb/off/1"))?;
            mouse.send_feature_report(get_dump!("../data/rgb/off/2"))?;
            mouse.send_feature_report(get_dump!("../data/rgb/off/3"))?;
        }
        RGBMode::Glorious => {
            mouse.send_feature_report(get_dump!("../data/rgb/glorious/1"))?;
            mouse.send_feature_report(get_dump!("../data/rgb/glorious/2"))?;
            mouse.send_feature_report(get_dump!("../data/rgb/glorious/3"))?;
        }
        RGBMode::Breathing => {
            mouse.send_feature_report(get_dump!("../data/rgb/breathing/1"))?;
            mouse.send_feature_report(get_dump!("../data/rgb/breathing/2"))?;
            mouse.send_feature_report(get_dump!("../data/rgb/breathing/3"))?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let api = HidApi::new()?;

    let mouse = api.open(args.vid, args.pid)?;

    let info = mouse.get_device_info()?;
    println!(
        "{}: {} ({}:{})",
        info.manufacturer_string().unwrap_or("Unknown"),
        info.product_string().unwrap_or("Unknown"),
        info.vendor_id(),
        info.product_id()
    );

    if let Some(mode) = args.rgb {
        set_rgb(&mouse, mode)?;
    }

    Ok(())
}
