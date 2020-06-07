use crate::oscillator::Oscillator;

pub struct Synth {
    oscillator: Oscillator,
    volume: f32
}

impl Synth {
    pub fn new() -> Synth {
        return Synth { oscillator: Oscillator::new(440.), volume: 1. };
    }

    pub fn play_note(&mut self, midi_note: u8) {
        let freq = 440. * f32::powf(2., (midi_note as f32 - 69.) / 12.);
        self.oscillator.set_frequency(freq);
    }

    pub fn output(&mut self, outbuf: &mut Vec<f32>, size: usize) {
        self.oscillator.output(outbuf, size);
    }

    pub fn get_volume(&mut self) -> f32 {
        return 0.;
    }
}
