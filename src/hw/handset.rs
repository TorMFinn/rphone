use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use std::error::Error;

#[derive(Debug)]
pub enum HandsetState {
    LIFTED,
    DOWN,
}

pub struct Config {
    gpio_dev: String,
    gpio_line: u32,
}

struct HandsetInternal {
    line_handle: LineHandle,
}

pub struct Handset {
    internal: HandsetInternal,
}

impl Handset {
    pub fn init(cfg: &Config) -> Result<Handset, Box<dyn Error>> {
        let mut chip = Chip::new(cfg.gpio_dev.as_str()).expect("failed to open gpio device");

        let line_result = chip
            .get_line(cfg.gpio_line)
            .expect("failed to get gpio line");

        let line_handle = line_result
            .request(LineRequestFlags::INPUT, 0, "read-input")
            .expect("failed to get line_request");

        let internal_handset = HandsetInternal { line_handle };
        Ok(Handset {
            internal: internal_handset,
        })
    }

    pub fn init_default() -> Result<Handset, Box<dyn Error>> {
        let default_cfg = Config {
            gpio_dev: String::from("/dev/gpiochip0"),
            gpio_line: 13,
        };

        Self::init(&default_cfg)
    }

    pub fn get_state(&self) -> HandsetState {
        let state = match self.internal.line_handle.get_value() {
            Ok(number) => {
                if number == 1 {
                    HandsetState::LIFTED
                } else {
                    HandsetState::DOWN
                }
            }
            Err(e) => {
                println!("Failed to get state: {}", e);
                HandsetState::DOWN
            }
        };

        state
    }
}
