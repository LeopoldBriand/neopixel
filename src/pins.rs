use arduino_hal::hal::port::{PB0, PB1, PB2, PB3, PB4, PB5, PC0, PC1, PC2, PC3, PC4, PC5, PD2, PD3, PD4, PD5, PD6, PD7};


pub trait NeoPixelPin {
    /// AVR data-space address of the PORT register.
    const PORT_ADDR: u16;

    /// One-hot mask for the GPIO bit.
    const MASK: u8;
}

impl NeoPixelPin for PD2 { // Pin D2
    const PORT_ADDR: u16 = 0x2B;
    const MASK: u8 = 0x04;
}

impl NeoPixelPin for PD3 { // Pin D3
    const PORT_ADDR: u16 = 0x2B;
    const MASK: u8 = 0x08;
}

impl NeoPixelPin for PD4 { // Pin D4
    const PORT_ADDR: u16 = 0x2B;
    const MASK: u8 = 0x10;
}

impl NeoPixelPin for PD5 { // Pin D5
    const PORT_ADDR: u16 = 0x2B;
    const MASK: u8 = 0x20;
}

impl NeoPixelPin for PD6 { // Pin D6
    const PORT_ADDR: u16 = 0x2B;
    const MASK: u8 = 0x40;
}

impl NeoPixelPin for PD7 { // Pin D7
    const PORT_ADDR: u16 = 0x2B;
    const MASK: u8 = 0x80;
}

impl NeoPixelPin for PB0 { // Pin D8
    const PORT_ADDR: u16 = 0x25;
    const MASK: u8 = 0x01;
}

impl NeoPixelPin for PB1 { // Pin D9
    const PORT_ADDR: u16 = 0x25;
    const MASK: u8 = 0x02;
}

impl NeoPixelPin for PB2 { // Pin D10
    const PORT_ADDR: u16 = 0x25;
    const MASK: u8 = 0x04;
}

impl NeoPixelPin for PB3 { // Pin D11
    const PORT_ADDR: u16 = 0x25;
    const MASK: u8 = 0x08;
}

impl NeoPixelPin for PB4 { // Pin D12
    const PORT_ADDR: u16 = 0x25;
    const MASK: u8 = 0x10;
}

impl NeoPixelPin for PB5 { // Pin D13
    const PORT_ADDR: u16 = 0x25;
    const MASK: u8 = 0x20;
}

impl NeoPixelPin for PC0 { // Pin A0
    const PORT_ADDR: u16 = 0x28;
    const MASK: u8 = 0x01;
}

impl NeoPixelPin for PC1 { // Pin A1
    const PORT_ADDR: u16 = 0x28;
    const MASK: u8 = 0x02;
}

impl NeoPixelPin for PC2 { // Pin A2
    const PORT_ADDR: u16 = 0x28;
    const MASK: u8 = 0x04;
}

impl NeoPixelPin for PC3 { // Pin A3
    const PORT_ADDR: u16 = 0x28;
    const MASK: u8 = 0x08;
}

impl NeoPixelPin for PC4 { // Pin A4
    const PORT_ADDR: u16 = 0x28;
    const MASK: u8 = 0x10;
}

impl NeoPixelPin for PC5 { // Pin A5
    const PORT_ADDR: u16 = 0x28;
    const MASK: u8 = 0x20;
}