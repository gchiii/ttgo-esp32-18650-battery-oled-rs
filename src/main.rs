use std::fmt::Write;

use esp_idf_svc::hal::{
    gpio::PinDriver,
    peripherals::Peripherals, i2c::I2cConfig,
    i2c::*,
    prelude::*,
    delay::FreeRtos,
};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio5;
    let scl = peripherals.pins.gpio4;

    let mut led_pin = PinDriver::output_od(peripherals.pins.gpio16).unwrap();

    if let Err(e) = led_pin.set_high() {
        log::error!("error {}", e);
    }
    let config = I2cConfig::new().baudrate(400.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();
    let mut interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_terminal_mode();

    display.init().unwrap();
    write!(display, "Hello, rust!").unwrap();

    log::info!("Hello, world!");
    loop {
        if let Err(e) = led_pin.set_level((!led_pin.is_set_high()).into()) {
            log::error!("error {}", e);
        }
        FreeRtos::delay_ms(1000);
    }
}
