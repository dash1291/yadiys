use crate::oscillator::Oscillator;

pub struct Synth {
    oscillators: Vec<Oscillator>,
    volume: f32,
    attack_time: Vec<f32>,
    release_time: Vec<f32>,
    decay_time: Vec<f32>,
    attack_interval: f32,
    release_interval: f32,
    decay_interval: f32,
    next_oscillator: usize,
    notes: Vec<u8>,
    sustain_value: f32,
}

impl Synth {
    pub fn new() -> Synth {
        return Synth {
            oscillators: vec![
                Oscillator::new(440.),
                Oscillator::new(440.),
                Oscillator::new(440.),
            ],
            volume: 1.,
            attack_time: vec![-1.0, -1.0, -1.0],
            release_time: vec![-1.0, -1.0, -1.0],
            decay_time: vec![-1.0, -1.0, -1.0],
            sustain_value: 0.5,
            notes: vec![69, 69, 69],
            attack_interval: 0.500,
            release_interval: 0.500,
            decay_interval: 0.5,
            next_oscillator: 0,
        };
    }

    pub fn set_note(&mut self, midi_note: u8) {
        let freq = 440. * f32::powf(2., (midi_note as f32 - 69.) / 12.);
        self.oscillators[self.next_oscillator].set_frequency(freq);
        self.notes[self.next_oscillator] = midi_note;
    }

    pub fn trigger_attack(&mut self, midi_note: u8) {
        self.set_note(midi_note);

        self.attack_time[self.next_oscillator] = 0.;
        self.release_time[self.next_oscillator] = -1.;

        self.next_oscillator = (self.next_oscillator + 1) % self.notes.len();
    }

    pub fn trigger_release(&mut self, midi_note: u8) {
        //self.set_volume(0.);
        for j in 0..(self.notes.len()) {
            if (self.notes[j] == midi_note) {
                self.attack_time[j] = -1.;
                self.release_time[j] = 0.
            }
        }
    }

    pub fn output(&mut self, outbuf: &mut [f32], size: usize) {
        for i in 0..(size) {
            outbuf[i] = 0.;

            let amp_per_voice = 1. / (self.notes.len() as f32);

            for j in 0..(self.oscillators.len()) {
                if (self.attack_time[j] >= 0.) {
                    self.attack_time[j] = self.attack_time[j] + (1. / 44100.);

                    self.oscillators[j].set_amplitude(
                        (self.attack_time[j] / self.attack_interval) * amp_per_voice,
                    );

                    if self.attack_time[j] > self.attack_interval {
                        self.attack_time[j] = -1.;
                    }
                } else if (self.release_time[j] >= 0.) {
                    self.release_time[j] = self.release_time[j] + (1. / 44100.);

                    self.oscillators[j].set_amplitude(
                        (1. - (self.release_time[j] / self.release_interval)) * amp_per_voice,
                    );

                    if self.release_time[j] > self.release_interval {
                        self.release_time[j] = -1.;
                    }
                } else if (self.decay_time[j] >= 0.) {
                    self.decay_time[j] = self.decay_time[j] + (1. / 44100.);

                    self.oscillators[j].set_amplitude(
                        self.sustain_value
                            * (1. - (self.decay_time[j] / self.decay_interval))
                            * amp_per_voice,
                    );

                    if self.decay_time[j] > self.decay_interval {
                        self.decay_time[j] = -1.;
                    }
                }
                outbuf[i] = outbuf[i] + self.oscillators[j].output();
            }

            outbuf[i] = self.volume * outbuf[i];
        }
    }

    pub fn get_volume(&mut self) -> f32 {
        return self.volume;
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol;
    }
}
