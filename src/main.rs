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

use rand::prelude::*;

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
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    let style = text_style!(
        font = Font6x8,
        text_color = Rgb565::GREEN,
        background_color = Rgb565::BLACK
    );

    let text = " Hi! ";

    let mut delay = McycleDelay::new(&rcu.clocks);

    let mut i = width / 2;

    delay.delay_ms(10000);

    // Just in case it doesn't already have a completely blank screen
    lcd.clear(Rgb565::BLACK)
        .expect("Failed to clear the screen");

    let mut rng = rand::rngs::SmallRng::from_seed([
        0x0, 0xD, 0xD, 0xB, 0x1, 0xA, 0x5, 0xE, 0x5, 0xB, 0xA, 0xD, 0x5, 0xE, 0xE, 0xD,
    ]);

    let mut y = rng.gen_range(0..height);

    loop {
        (0..text.len()).for_each(|idx| {
            Text::new(
                &text[idx..=idx],
                Point::new((i + (idx * 6) as i32) % width, y),
            )
            .into_styled(style)
            .draw(&mut lcd)
            .unwrap()
        });

        if i == 0 {
            lcd.clear(Rgb565::BLACK)
                .expect("Failed to clear the screen");
            y = rng.gen_range(0..height);
            i = width;
        } else {
            i -= 1;
        }

        delay.delay_ms(16);
    }
}
