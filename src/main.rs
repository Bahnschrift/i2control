mod dump;

use clap::{Parser, ValueEnum};
use std::error::Error;

use hidapi::{HidApi, HidDevice};

const I2_VID: u16 = 0x93A;
const I2_PID: u16 = 0x821D;

#[derive(Debug, Parser)]
struct Args {
    #[arg(default_value_t = I2_VID)]
    vid: u16,

    #[arg(default_value_t = I2_PID)]
    pid: u16,

    #[arg(short, long)]
    rgb: Option<RGBMode>,

    /// Polling rate (in Hz). Must be one of 125, 250, 500, or 1000.
    #[arg(short, long)]
    polling_rate: Option<u16>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum RGBMode {
    Off,
    Glorious,
    SeamlessBreathing,
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
        RGBMode::SeamlessBreathing => {
            mouse.send_feature_report(get_dump!("../data/rgb/seamless_breathing/1"))?;
            mouse.send_feature_report(get_dump!("../data/rgb/seamless_breathing/2"))?;
            mouse.send_feature_report(get_dump!("../data/rgb/seamless_breathing/3"))?;
        },
    }

    Ok(())
}

fn set_polling_rate(mouse: &HidDevice, rate: u16) -> Result<(), Box<dyn Error>> {
    match rate {
        1000 => {
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/1"))?;
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/2"))?;
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/3"))?;
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/4"))?;
        }
        500 => {
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/1"))?;
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/2"))?;
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/3"))?;
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/4"))?;
        }
        250 => {
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/1"))?;
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/2"))?;
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/3"))?;
            mouse.send_feature_report(get_dump!("../data/polling/1000Hz/4"))?;
        }
        125 => todo!(),
        _ => return Err("Invalid polling rate. Must be one of 125, 250, 500, 1000".into()),
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

    if let Some(rate) = args.polling_rate {
        set_polling_rate(&mouse, rate)?;
    }

    Ok(())
}
