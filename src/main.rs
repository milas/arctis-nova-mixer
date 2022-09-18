mod winmixer;
use hidapi::HidApi;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    unsafe {
        winmixer::mixer()?;
    }

    loop {
        let mut buf = [0, 0, 0];
        device.read(&mut buf)?;
        // device.get_feature_report(&mut buf).unwrap();
        println!("Game Volume: {} | Chat Volume: {}", buf[1], buf[2]);
    }
}
