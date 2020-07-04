pub struct Oscillator {
    frequency: f32,
    current_phase: f32,
    amplitude: f32
}

impl Oscillator {
    pub fn new(frequency: f32) -> Oscillator {
        return Oscillator {
            frequency,
            current_phase: 0.,
            amplitude: 0.,
        };
    }

    pub fn output(&mut self) -> f32 {
        self.current_phase = (self.current_phase + self.frequency / 44100.) % 1.;
        return f32::sin(2. * 3.14159265 * self.current_phase) * self.amplitude;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn set_amplitude(&mut self, amp: f32) {
        self.amplitude = amp;
    }
}
