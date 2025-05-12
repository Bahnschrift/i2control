mod cli;
mod util;
mod lighting;

use clap::Parser;
use cli::args::{Args, RGBMode};
use std::error::Error;

use hidapi::{HidApi, HidDevice};

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
        }
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
