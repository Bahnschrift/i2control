mod cli;

use std::error::Error;

use clap::Parser;
use hidapi::{DeviceInfo, HidApi};

use cli::{Cli, Commands};
use i2control::device::{dpi::set_dpi, lighting::set_lighting, timeout::set_timeout};

fn print_device_info(info: &DeviceInfo) {
    println!(
        "{}: {} ({}:{})",
        info.manufacturer_string().unwrap_or("Unknown"),
        info.product_string().unwrap_or("Unknown"),
        info.vendor_id(),
        info.product_id()
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let api = HidApi::new()?;

    dbg!(&args);

    let mouse = api.open(args.vid, args.pid)?;
    let info = mouse.get_device_info()?;
    print_device_info(&info);

    match args.command {
        Commands::Lighting {
            brightness,
            rate,
            mode,
        } => set_lighting(&mouse, brightness, rate, mode)?,
        Commands::Dpi {
            lift_off_distance,
            debounce_time,
            polling_rate,
            dpi_stages,
        } => {
            set_dpi(
                &mouse,
                lift_off_distance,
                debounce_time,
                polling_rate,
                dpi_stages,
            )?;
        }
        Commands::Timeout {
            disable: _,
            minutes,
        } => set_timeout(&mouse, minutes)?,
        _ => todo!(),
    }

    Ok(())
}
