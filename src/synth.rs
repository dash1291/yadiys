use crate::oscillator::Oscillator;

pub struct Synth {
    oscillator: Oscillator,
    volume: f32,
}

impl Synth {
    pub fn new() -> Synth {
        return Synth {
            oscillator: Oscillator::new(440.),
            volume: 0.,
        };
    }

    pub fn set_note(&mut self, midi_note: u8) {
        let freq = 440. * f32::powf(2., (midi_note as f32 - 69.) / 12.);
        self.oscillator.set_frequency(freq);
    }

    pub fn trigger_attack(&mut self, midi_note: u8) {
        self.set_volume(1.);
        self.set_note(midi_note);
    }

    pub fn trigger_release(&mut self) {
        self.set_volume(0.);
    }

    pub fn output(&mut self, outbuf: &mut [f32], size: usize) {
        for i in 0..size {
            if self.volume > 0. {
                outbuf[i] = 5.0 * self.oscillator.output();
            } else {
                outbuf[i] = 0.;
            }
        }
    }

    pub fn get_volume(&mut self) -> f32 {
        return self.volume;
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol;
    }
}
