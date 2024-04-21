use clap::{Arg, Command};
use std::fs::File;
use std::io::Write;
use std::path::Path;

macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

fn main() {
    let matches = Command::new("pwr-usb switchbox - usb switchbox control")
        .about("Controls pwr-usb switchbox.On/Off swichbox usb and power ports")
        .disable_version_flag(true)
        .arg(
            Arg::new("tty")
                .help("Four port usb-pwr switchbox in use")
                .use_value_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::new("number")
                .help("Port number")
                .use_value_delimiter(false)
                .required(true)
                .takes_value(true)
                .possible_values(["1", "2", "3", "4"]),
        )
        .arg(
            Arg::new("power")
                .help("Enable power")
                .use_value_delimiter(false)
                .required(false)
                .possible_values(["0", "1"])
                .default_value("0"),
        )
        .arg(
            Arg::new("usb")
                .help("Enable usb")
                .use_value_delimiter(false)
                .required(false)
                .possible_values(["0", "1"])
                .default_value("0"),
        )
        .get_matches();

    let tty_name = matches.value_of("tty").unwrap();
    let port_number = matches.value_of("number").unwrap().parse::<u8>().unwrap();
    let power_port = matches.value_of("power").unwrap().parse::<u8>().unwrap();
    let usb_port = matches.value_of("usb").unwrap().parse::<u8>().unwrap();

    // Create a path to the switchbox tty file
    let path = Path::new(tty_name);
    let display = path.display();

    let write_command: u8 = 0xc0;
    let read_command: u8 = 0x80;

    let mut set_ports: u8 = 0;
    let mut port: u8 = 0;
    port |= 1 << (port_number - 1);

    // set power ports
    if power_port == 1 {
        set_ports |= port;
    }

    // set usb ports
    if usb_port == 1 {
        set_ports |= port << 4;
    }

    // Open switchbox tty device in read/write mode, returns `io::Result<File>`
    let mut tty_device = match File::options().read(true).write(true).open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Write `0xc0` to `tty_device`, returns `io::Result<()>`
    match tty_device.write(&[write_command]) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => debug_println!("successfully wrote 0xc0 to {}", display),
    }

    // Write `set_ports` to `tty_device`, returns `io::Result<()>`
    match tty_device.write(&[set_ports]) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => debug_println!(
            "successfully wrote {} to {}",
            format!("{set_ports:b}"),
            display
        ),
    }

    // Write `0x80` to `tty_device`, returns `io::Result<()>`
    match tty_device.write(&[read_command]) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => debug_println!("successfully wrote 0x80 to {}", display),
    }

    let _ = tty_device.flush();

    // `tty_device` goes out of scope, and the the file gets closed
}
