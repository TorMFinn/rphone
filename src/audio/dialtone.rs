use libpulse_binding::sample::{Format, Spec};
use libpulse_binding::stream::Direction;
use libpulse_simple_binding::Simple;

use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

struct DialtoneInternal {
    pasimple: Simple,
    paspec: Spec,
    run: bool,
}

pub struct Dialtone {
    internal: Arc<Mutex<DialtoneInternal>>,
    audio_thd: Option<JoinHandle<()>>,
}

impl Dialtone {
    pub fn init() -> Result<Dialtone, Box<dyn Error>> {
        let spec = Spec {
            format: Format::S16NE,
            channels: 1,
            rate: 8000,
        };
        assert!(spec.is_valid());

        let s = Simple::new(
            None,
            "rphone",
            Direction::Playback,
            None,
            "dialtone",
            &spec,
            None,
            None,
        )
        .expect("Failed to create pa simple");

        let internal = DialtoneInternal {
            pasimple: s,
            paspec: spec,
            run: false,
        };

        Ok(Dialtone {
            internal: Arc::new(Mutex::new(internal)),
            audio_thd: None,
        })
    }

    pub fn start(&mut self) {
	self.internal.lock().unwrap().run = true;

        let internal_cpy = self.internal.clone();

        let thd_handle = std::thread::spawn(move || {
            let bufsize = 512;
            let mut audio_buf = Vec::<i16>::with_capacity(bufsize);
            let mut t = 0f32;

            loop {
                let internal = internal_cpy.lock().unwrap();
                if !internal.run {
		    break;
		}
                for _ in 0..bufsize {
                    audio_buf.push(((std::i16::MAX) as f32 * (t * 420.0).sin()) as i16);
                    t += (2.0 * std::f32::consts::PI) / (internal.paspec.rate as f32);

                    if t >= 2.0 * std::f32::consts::PI {
                        t = -(2.0 * std::f32::consts::PI);
                    }
                }

                unsafe {
                    let (_, middle, _) = audio_buf.align_to();

                    match internal.pasimple.write(&middle) {
                        Ok(_) => println!("Data written"),
                        Err(pa_error) => println!("Failed to write data: {}", pa_error),
                    }

                    audio_buf.clear();
                }
            }
        });

        self.audio_thd = Some(thd_handle);
    }

    pub fn stop(self) {
        let mut internal = self.internal.lock().unwrap();
        internal.run = false;
        self.audio_thd
            .unwrap()
            .join()
            .expect("failed to join thread");
    }
}
