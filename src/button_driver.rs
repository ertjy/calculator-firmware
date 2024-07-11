use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::Infallible;
use debouncr::{debounce_16, debounce_stateful_16, Debouncer, DebouncerStateful, Edge, Repeat16};
use embedded_hal::digital::v2::{InputPin, OutputPin};

const ROWS: usize = 9;
const COLUMNS: usize = 3;

const BUTTONS: [[ButtonClick; COLUMNS]; ROWS] = [
    [
        ButtonClick::Digit7,
        ButtonClick::Digit8,
        ButtonClick::Digit9,
    ],
    [
        ButtonClick::Digit4,
        ButtonClick::Digit5,
        ButtonClick::Digit6,
    ],
    [
        ButtonClick::Digit1,
        ButtonClick::Digit2,
        ButtonClick::Digit3,
    ],
    [ButtonClick::Digit0, ButtonClick::Dot, ButtonClick::Answer],
    [
        ButtonClick::Add,
        ButtonClick::Subtract,
        ButtonClick::LeftParen,
    ],
    [
        ButtonClick::RightParen,
        ButtonClick::Ignored,
        ButtonClick::Ignored,
    ],
    [
        ButtonClick::Ignored,
        ButtonClick::Ignored,
        ButtonClick::Ignored,
    ],
    [
        ButtonClick::Ignored,
        ButtonClick::Ignored,
        ButtonClick::Ignored,
    ],
    [
        ButtonClick::Ignored,
        ButtonClick::Ignored,
        ButtonClick::Ignored,
    ],
];

pub struct ButtonDriver {
    col_pins: Vec<Box<dyn OutputPin<Error = Infallible>>>,
    row_pins: Vec<Box<dyn InputPin<Error = Infallible>>>,
    debouncers: [[Debouncer<u16, Repeat16>; COLUMNS]; ROWS],
}

#[derive(Copy, Clone, Debug)]
pub enum ButtonClick {
    Ignored,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Answer,
    Add,
    Subtract,
    Dot,
    LeftParen,
    RightParen,
}

impl ButtonDriver {
    pub fn new(
        col_pins: Vec<Box<dyn OutputPin<Error = Infallible>>>,
        row_pins: Vec<Box<dyn InputPin<Error = Infallible>>>,
    ) -> Self {
        let debouncers = core::array::from_fn(|_i| core::array::from_fn(|_i| debounce_16(false)));
        Self {
            col_pins,
            row_pins,
            debouncers,
        }
    }

    pub fn get_clicks(&mut self) -> Vec<ButtonClick> {
        let mut clicks = vec![];
        for col_pin_index in 0..self.col_pins.len() {
            let col_pin = &mut self.col_pins[col_pin_index];

            col_pin.set_high().unwrap();
            for row_pin_index in 0..self.row_pins.len() {
                let row_pin = &self.row_pins[row_pin_index];
                if let Some(Edge::Rising) =
                    self.debouncers[row_pin_index][col_pin_index].update(row_pin.is_high().unwrap())
                {
                    clicks.push(BUTTONS[row_pin_index][col_pin_index]);
                }
            }
            col_pin.set_low().unwrap();
        }
        clicks
    }
}
