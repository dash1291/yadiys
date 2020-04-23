use std::{thread, time};


use crate::oscillator::Oscillator;


pub struct Synth {
    oscillator: Oscillator,
    volume: f32
}

impl Synth {
    pub fn new() -> Synth {
        return Synth { oscillator: Oscillator::new(440.), volume: 1. };
    }

    pub fn play_note(&mut self) {
        self.oscillator.set_frequency(440.)
    }

    pub fn output(&mut self, outbuf: &mut [f32], size: usize) {
        self.oscillator.output(outbuf, size);
    }

    pub fn get_oscillator(&self) -> &Oscillator {
        return &self.oscillator;
    }

    pub fn get_volume(&mut self) -> f32 {
        return 0.;
    }
}
