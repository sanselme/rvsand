#[allow(unused_imports)]
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use esp_idf_hal::{i2c, peripherals::Peripherals, units::FromValueType};
use esp_idf_sys as _;
#[allow(unused_imports)]
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Configuring peripherals...");
    let peripherals = Peripherals::take().unwrap();
    let sda = peripherals.pins.gpio4;
    let scl = peripherals.pins.gpio5;

    println!("Configuring I2C...");
    let i2c = i2c::I2cDriver::new(
        peripherals.i2c0,
        sda,
        scl,
        &i2c::I2cConfig::new()
            .baudrate(400.kHz().into())
            .sda_enable_pullup(true)
            .scl_enable_pullup(true),
    )
    .unwrap();

    println!("Configuring display...");
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    println!("Initializing display...");
    match display.init() {
        Ok(_) => println!("Display initialized"),
        Err(e) => println!("Display initialization failed: {:?}", e),
    };

    println!("Configuring text style...");
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    println!("Drawing text...");
    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();
    Text::with_baseline("Hello Rust!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    println!("Flushing display...");
    match display.flush() {
        Ok(_) => println!("Display flushed"),
        Err(e) => println!("Display flush failed: {:?}", e),
    };

    // SSD1306
    //  - i2c enable
    //  - SSD1306 display
    //  - display some text

    println!("Hello, world!");
}
