extern crate sdl2;

use rand::prelude::*;
use sdl2::audio::{AudioCallback, AudioSpec, AudioSpecDesired};
use std::time::{Duration, Instant};

struct SquareWave {
    volume: f32,
    spec: AudioSpec,
    sound_time: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
            let beat = 50;
            let bar = beat * 4;

            let f0 = ((self.sound_time as u64 % bar * 4) as f32 / bar as f32) * rand::thread_rng().gen_range(100.0..600.0);

            for (i, x) in out.iter_mut().enumerate() {
                let harmonic_count = (self.sound_time as u64 % bar / beat * 5) + 10;
                for harmonic in 1..harmonic_count {
                    let f_local = f0 * harmonic as f32;

                    *x += ((f_local / self.spec.freq as f32) * i as f32).sin() / harmonic as f32;
                }
                // *x += (i as f32 * (200.0 / self.spec.freq as f32)).sin();
                *x /= harmonic_count as f32;
                *x *= self.volume;
                self.sound_time += (self.spec.samples as f32 / self.spec.freq as f32) * 1000.0;
            }

    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1),    // mono
        samples: Some(2048), // default sample size
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // Show obtained AudioSpec
        println!("{:?}", spec);

        // initialize the audio callback
        SquareWave {
            volume: 1.0,
            spec,
            sound_time: 0.0,
        }
    })?;

    device.resume();
    loop {}
}
