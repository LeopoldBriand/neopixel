#![no_std]
#![no_main]


use panic_halt as _;

use neopixel::NeoPixel;
use smart_leds::RGB8;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let data_pin = pins.d11.into_output();
    const NUMPIXELS: usize = 3;

    let mut strip = NeoPixel::<_, NUMPIXELS>::new(data_pin);

    strip.set_brightness(150);

    loop {
        for i in 0..NUMPIXELS {
            strip.set_pixel_color(i, RGB8 { r: 255, g: 0, b: 0 });
        }
        strip.show();
        arduino_hal::delay_ms(1000);

        for i in 0..NUMPIXELS {
            strip.set_pixel_color(i, RGB8 { r: 0, g: 0, b: 0 });
        }
        strip.show();
        arduino_hal::delay_ms(1000);
    }
}