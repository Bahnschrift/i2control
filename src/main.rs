// TODO: Proper project structure - may need a workspace
mod cli;
mod lighting;
mod report;
mod util;

use clap::Parser;
use cli::{Args, Commands};
use lighting::set_lighting;
use std::error::Error;

use hidapi::{DeviceInfo, HidApi};

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
    let args = Args::parse();
    let api = HidApi::new()?;

    let mouse = api.open(args.vid, args.pid)?;
    let info = mouse.get_device_info()?;
    print_device_info(&info);

    match args.command {
        Commands::Lighting {
            brightness,
            rate,
            mode,
        } => set_lighting(&mouse, brightness, rate, mode)?,
        _ => todo!(),
    }

    Ok(())
}
