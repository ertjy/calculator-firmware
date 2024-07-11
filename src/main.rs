#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::alloc::Layout;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::convert::Infallible;
use panic_rtt_target as _;

use calculator_firmware_library::button_driver::ButtonDriver;
use calculator_firmware_library::math_driver::MathDriver;
use cortex_m_rt::entry;
use debouncr::{debounce_16, debounce_stateful_16};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use fugit::HertzU32;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::device::{CorePeripherals, Peripherals};
use stm32f1xx_hal::gpio::Input;
use stm32f1xx_hal::prelude::*;

#[global_allocator]
pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    panic!("Ran out of memory");
}

pub fn allocate_heap(heap_size: usize, allocator: &'static CortexMHeap) {
    let start = cortex_m_rt::heap_start() as usize;
    unsafe { allocator.init(start, heap_size) }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(HertzU32::MHz(8))
        .sysclk(HertzU32::MHz(72))
        .hclk(HertzU32::MHz(72))
        .pclk1(HertzU32::MHz(36))
        .pclk2(HertzU32::MHz(72));

    allocate_heap(4096, &ALLOCATOR);

    let mut gpioc = dp.GPIOC.split();

    let mut col_pins: Vec<Box<dyn OutputPin<Error = Infallible>>> = vec![
        Box::new(gpioc.pc0.into_push_pull_output(&mut gpioc.crl)),
        Box::new(gpioc.pc1.into_push_pull_output(&mut gpioc.crl)),
        Box::new(gpioc.pc2.into_push_pull_output(&mut gpioc.crl)),
    ];

    let mut row_pins: Vec<Box<dyn InputPin<Error = Infallible>>> = vec![
        Box::new(gpioc.pc3.into_pull_down_input(&mut gpioc.crl)),
        Box::new(gpioc.pc4.into_pull_down_input(&mut gpioc.crl)),
        Box::new(gpioc.pc5.into_pull_down_input(&mut gpioc.crl)),
        Box::new(gpioc.pc6.into_pull_down_input(&mut gpioc.crl)),
        Box::new(gpioc.pc7.into_pull_down_input(&mut gpioc.crl)),
        Box::new(gpioc.pc8.into_pull_down_input(&mut gpioc.crh)),
        Box::new(gpioc.pc9.into_pull_down_input(&mut gpioc.crh)),
        Box::new(gpioc.pc10.into_pull_down_input(&mut gpioc.crh)),
        Box::new(gpioc.pc11.into_pull_down_input(&mut gpioc.crh)),
    ];
    let mut button_driver = ButtonDriver::new(col_pins, row_pins);
    let mut math_driver = MathDriver::new();

    loop {
        let clicks = button_driver.get_clicks();
        for click in clicks.iter() {
            math_driver.handle_click(click);
        }
    }
}
