// Copyright (c) 2022 Schubert Anselme <schubert@anselm.es>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use esp_idf_hal::{delay::FreeRtos, ledc::*, peripherals::Peripherals, prelude::*};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
#[allow(unused_imports)]
use smart_leds::{
    brightness, gamma,
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
};

fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Configuring output channel");
    let peripherals = Peripherals::take().unwrap();
    let mut led = LedcDriver::new(
        peripherals.ledc.channel0,
        LedcTimerDriver::new(
            peripherals.ledc.timer0,
            &config::TimerConfig::new().frequency(25.kHz().into()),
        )?,
        peripherals.pins.gpio8,
    )?;

    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };
    // let mut data; # FIXME:

    loop {
        println!("Starting duty cycle");
        let max_duty = led.get_max_duty();
        for numerator in [0, 1, 2, 3, 4, 5].iter().cycle() {
            println!("Duty {}/5", numerator);
            led.set_duty(max_duty * numerator / 5)?;
            FreeRtos::delay_ms(2000);
        }

        println!("Displaying color");
        for hue in 0..=255 {
            color.hue = hue;

            // FIXME:
            // Convert from the HSV color space (where we can easily transition from one
            // color to the other) to the RGB color space that we can then send to the LED
            // data = [hsv2rgb(color)];

            // FIXME:
            // When sending to the LED, we do a gamma correction first (see smart_leds
            // documentation for details) and then limit the brightness to 10 out of 255 so
            // that the output it's not too bright.
            // SmartLedsWrite::write(&mut led, brightness(gamma(data.iter().cloned()), 10))?;

            FreeRtos::delay_ms(1000);
        }
    }
}
