# pwr-usb-switchbox
PWR-USB Switchbox  - One of those old hardware that happen to be in my storage (good old Nokia memories).
It is a device that provides 4-channel USB 2.0 switching as well as 4 channel power distribution capabilities. 

URL: http://regius.ee/pwr-usb-switchbox/

Decided to reuse that hardware and learn some Rust on the way.   

```
» ./pwr-usb-switchbox -h
pwr-usb switchbox - usb switchbox control 
Controls pwr-usb switchbox.On/Off swichbox usb and power ports

USAGE:
    pwr-usb-switchbox <tty> <number> [ARGS]

ARGS:
    <tty>       Four port usb-pwr switchbox in use
    <number>    Port number [possible values: 1, 2, 3, 4]
    <power>     Enable power [default: 0] [possible values: 0, 1]
    <usb>       Enable usb [default: 0] [possible values: 0, 1]

OPTIONS:
    -h, --help    Print help information

 » ./pwr-usb-switchbox /dev/ttyUSB0 3 1 1
successfully wrote 0xc0 to /dev/ttyUSB0
successfully wrote 1000100 to /dev/ttyUSB0
successfully wrote 0x80 to /dev/ttyUSB0

```
