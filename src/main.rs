#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

const RCC_BASE: u32 = 0x4002_3800;
const RCC_AHB1ENR_OFFSET: u32 = 0x30;
const GPIOB_BASE: u32 = 0x4002_0400;
const GPIOB_MODER_OFFSET: u32 = 0x00;
const GPIOB_BSRR_OFFSET: u32 = 0x18;

#[entry]
fn main() -> ! {
    // Enable clock for GPIOB
    let rcc_ahb1enr = (RCC_BASE + RCC_AHB1ENR_OFFSET) as *mut u32;
    unsafe { *rcc_ahb1enr |= 1 << 1; } // GPIOBEN

    // Set GPIOB pin 0 as output
    let gpio_moder = (GPIOB_BASE + GPIOB_MODER_OFFSET) as *mut u32;
    unsafe { *gpio_moder |= 1 << (0 * 2); } // Set MODER0[1:0] to 01 (Output mode)

    let gpio_bsrr = (GPIOB_BASE + GPIOB_BSRR_OFFSET) as *mut u32;

    loop {
        // Turn LED on
        unsafe { *gpio_bsrr = 1 << 0; } // Set BS0

        // Simple delay
        for _ in 0..1_000_000 {}

        // Turn LED off
        unsafe { *gpio_bsrr = 1 << 16; } // Set BR0

        // Simple delay
        for _ in 0..1_000_000 {}
    }
}