mod mixer;

#[macro_use]
extern crate lazy_static;

use crate::mixer::windows::WindowsMixer;
use crate::mixer::Mixer;

use hidapi::HidApi;
use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("pid").multiple(true).required(true).args(&["chat-pid", "game-pid"])))]
struct Args {
    /// PID of process to target for chat volume mix
    #[clap(long, value_parser, group = "pid")]
    chat_pid: Option<u64>,

    /// PID of process to target for game volume mix
    #[clap(long, value_parser, group = "pid")]
    game_pid: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let api = HidApi::new()?;
    let device = api
        .device_list()
        .find(|&device| {
            device.vendor_id() == 0x1038
                && device.product_id() == 0x2206
                && device.usage_page() == 0xFF00
        })
        .ok_or("no headset found")?
        .open_device(&api)?;

    let mixer = WindowsMixer::new()?;

    loop {
        let mut buf = [0, 0, 0];
        device.read(&mut buf)?;
        // device.get_feature_report(&mut buf).unwrap();
        let game_vol = buf[1];
        let chat_vol = buf[2];
        println!("Game Volume: {} | Chat Volume: {}", game_vol, chat_vol);

        if let Some(chat_pid) = args.chat_pid {
            mixer.set_volume(chat_pid, chat_vol as f64 / 100 as f64)?;
        }

        if let Some(game_pid) = args.game_pid {
            mixer.set_volume(game_pid, game_vol as f64 / 100 as f64)?;
        }
    }
}
