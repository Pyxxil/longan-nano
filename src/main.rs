#![no_std]
#![no_main]
#![feature(slice_fill)]

use panic_halt as _;

use embedded_graphics::fonts::{Font6x8, Text};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text_style;
use gd32vf103xx_hal::delay::McycleDelay;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::prelude::*;
use longan_nano::{lcd, lcd_pins};
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();
    let mut afio = dp.AFIO.constrain(&mut rcu);

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpiob = dp.GPIOB.split(&mut rcu);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(dp.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, _) = (lcd.size().width as i32, lcd.size().height as i32);

    let style = text_style!(
        font = Font6x8,
        text_color = Rgb565::GREEN,
        background_color = Rgb565::BLACK
    );

    let text = "Hi!";

    let mut delay = McycleDelay::new(&rcu.clocks);

    let mut i = width / 2;

    loop {
        lcd.clear(Rgb565::BLACK).unwrap();

        (0..text.len())
            .map(|idx| {
                Text::new(
                    &text[idx..idx + 1],
                    Point::new((i + (idx * 6) as i32) % width, 35),
                )
                .into_styled(style)
            })
            .for_each(|ch| ch.draw(&mut lcd).unwrap());

        i = if i - 6 < 0 { width } else { i - 6 };

        delay.delay_ms(50);
    }
}
