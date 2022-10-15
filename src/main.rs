//use hw::handset::Handset;
use std::time::Duration;
use audio::dialtone::Dialtone;

//use crate::hw::handset::HandsetState;

use libpulse_binding::sample::{Format, Spec};
use libpulse_binding::stream::Direction;
use libpulse_simple_binding::Simple;

mod hw;
mod audio;

fn main() {
    //let handset = Handset::init_default().expect("Failed to init handset");

    let mut dialtone = Dialtone::init().expect("Failed to init dialtone audio");
    dialtone.start();

    std::thread::sleep(Duration::from_secs(2));

    dialtone.stop();

    /*
    loop {
    let state = handset.get_state();
    match state {
        HandsetState::LIFTED => {
        dialtone.start();
        },
        HandsetState::DOWN => {
        dialtone.stop();
        }
    }
    println!("State of handset is: {:?}", state);
    std::thread::sleep(Duration::from_millis(200));
    }
    */

    //dialtone.start();

    // Format for regular phone coms.
}
