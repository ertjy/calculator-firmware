use crate::button_driver::ButtonClick;
use alloc::string::{String, ToString};
use caldyn::eval;
use core::ops::AddAssign;
use rtt_target::rprintln;

pub struct MathDriver {
    input_buffer: String,
    output_buffer: String,
}

impl MathDriver {
    pub fn new() -> Self {
        Self {
            input_buffer: "".to_string(),
            output_buffer: "".to_string(),
        }
    }

    pub fn handle_click(&mut self, button_click: &ButtonClick) {
        match button_click {
            ButtonClick::Ignored => {}
            ButtonClick::Digit0 => self.input_buffer.add_assign("0"),
            ButtonClick::Digit1 => self.input_buffer.add_assign("1"),
            ButtonClick::Digit2 => self.input_buffer.add_assign("2"),
            ButtonClick::Digit3 => self.input_buffer.add_assign("3"),
            ButtonClick::Digit4 => self.input_buffer.add_assign("4"),
            ButtonClick::Digit5 => self.input_buffer.add_assign("5"),
            ButtonClick::Digit6 => self.input_buffer.add_assign("6"),
            ButtonClick::Digit7 => self.input_buffer.add_assign("7"),
            ButtonClick::Digit8 => self.input_buffer.add_assign("8"),
            ButtonClick::Digit9 => self.input_buffer.add_assign("9"),
            ButtonClick::Add => self.input_buffer.add_assign(" + "),
            ButtonClick::Subtract => self.input_buffer.add_assign(" - "),
            ButtonClick::LeftParen => self.input_buffer.add_assign("("),
            ButtonClick::RightParen => self.input_buffer.add_assign(")"),
            ButtonClick::Dot => self.input_buffer.add_assign("."),
            ButtonClick::Answer => self.solve(),
        }
        rprintln!("{}, {}", self.input_buffer, self.output_buffer);
    }

    fn solve(&mut self) {
        match eval(self.input_buffer.as_str(), None) {
            Ok(answer) => self.output_buffer = answer.to_string(),
            Err(err) => {
                rprintln!("{}", err);
                return;
            }
        }
    }
}
