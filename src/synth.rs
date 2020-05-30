use crate::oscillator::Oscillator;

pub struct Synth {
    oscillator: Oscillator,
    volume: f32,
}

impl Synth {
    pub fn new() -> Synth {
        return Synth {
            oscillator: Oscillator::new(440.),
            volume: 1.,
        };
    }

    pub fn play_note(&mut self, midi_note: u8) {
        let freq = 440. * f32::powf(2., (midi_note as f32 - 69.) / 12.);
        self.oscillator.set_frequency(freq);
    }

    pub fn output(&mut self, outbuf: &mut [f32], size: usize) {
        for i in 0..size {
            outbuf[i] = 5.0 * self.oscillator.output();
        }
    }

    pub fn get_volume(&mut self) -> f32 {
        return self.volume;
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol;
    }
}
