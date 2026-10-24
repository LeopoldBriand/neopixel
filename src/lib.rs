#![no_std]

use core::ptr::{read_volatile, write_volatile};

use arduino_hal::port::{Pin, PinOps, mode::Output};
use arduino_hal::avr_device;
use smart_leds::RGB8;

use crate::pins::NeoPixelPin;
use crate::tx_buffer::{TxBuf, TxConfig};

mod tx_buffer;
mod pins;

unsafe extern "C" {
    fn neopixel_tx(data: *const TxConfig);
}
pub struct NeoPixel<P, const N: usize> {
    pixels: [RGB8; N],
    pin: Pin<Output,P>,
    brightness: u8
}

impl<P, const N: usize> NeoPixel<P, N> 
    where 
        P: PinOps + NeoPixelPin
    {

    pub fn new(pin: Pin<Output,P>) -> Self {
        Self {
            pixels: [RGB8 { r: 0, g: 0, b: 0 }; N],
            pin,
            brightness: 255
        }
    }

    pub fn begin(&mut self) {
        self.pin.set_low();
    }

    pub fn show(&mut self){

        let mut buffer = TxBuf {
            bytes: [[0u8; 3]; N],
            pad: 0,
        };

        // Into GRB format, maybe handle different formats here
        for (dst, pixel) in buffer.bytes.iter_mut().zip(self.pixels.iter()) {
            dst[0] = self.scale(pixel.g);
            dst[1] = self.scale(pixel.r);
            dst[2] = self.scale(pixel.b);
        }

        let port = P::PORT_ADDR as *mut u8;

        // Disable interupts while transmission
        avr_device::interrupt::free(|_| {
            unsafe {
                let current = read_volatile(port);
                let lo = current & !P::MASK;
                let hi = current | P::MASK;

                // Establish a known low output state before transmission.
                write_volatile(port, lo);

                let config = TxConfig {
                    data: buffer.bytes.as_ptr().cast::<u8>(),
                    len: (N * 3) as u16,
                    port,
                    hi,
                    lo,
                };

                neopixel_tx(&config);
            }
        });

        // Interrupts have been restored. Allow the LEDs to latch.
        arduino_hal::delay_ms(1);


    }

    pub fn set_brightness(&mut self, b:u8){
        self.brightness = b;
    }

    fn scale(&self, value: u8) -> u8 {
        ((value as u16 * self.brightness as u16) / 255) as u8
    }

    pub fn set_pixel_color(&mut self, n: usize, color: RGB8){
        if let Some(pixel) = self.pixels.get_mut(n) {
            *pixel = color;
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = RGB8 { r: 0, g: 0, b: 0 };
        }
    }
}
