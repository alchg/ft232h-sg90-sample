use embedded_hal::blocking::spi::Write;

fn get_stream(angle: f32) -> Vec<u8> {
    let min_pulse_width = 0.5; // ms
    let max_pulse_width = 2.4; // ms
    let angle = angle.min(180.0).max(0.0);
    println!("angle:{}", angle);

    let pulse_width = min_pulse_width + (max_pulse_width - min_pulse_width) * (angle / 180.0);
    println!("pulse_width:{} ms", pulse_width);
    let period = 200; // 20ms

    let on_time = (pulse_width * 10.0) as u64;
    let off_time = period - on_time;

    //println!("on_time:{}", on_time);
    //println!("off_time:{}", off_time);

    let mut stream = Vec::new();

    for _ in 0..50 {
        stream.extend(vec![0xff; on_time as usize]);
        stream.extend(vec![0x00; off_time as usize]);
    }

    stream
}

fn main() {
    let device = ftdi::find_by_vid_pid(0x403, 0x6014)
        .interface(ftdi::Interface::A)
        .open()
        .unwrap();
    let hal = ftdi_embedded_hal::FtHal::init_freq(device, 80_000).unwrap();
    let mut spi = hal.spi().unwrap();

    spi.write(get_stream(0.0).as_slice()).unwrap();
    spi.write(get_stream(90.0).as_slice()).unwrap();
    spi.write(get_stream(180.0).as_slice()).unwrap();
}
