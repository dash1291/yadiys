pub struct Oscillator {
    frequency: f32,
    current_phase: f32
}

impl Oscillator {
    pub fn new(frequency: f32) -> Oscillator {
        return Oscillator { frequency, current_phase: 0. }
    }

    pub fn output(&mut self, outbuf: &mut Vec<f32>, size: usize) {
        for i in 0..(size - 1) {
            self.current_phase = (self.current_phase + self.frequency / 44100.) % 1.;
            outbuf[i] = f32::sin(2. * 3.14159265 * self.current_phase);
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency
    }

    
}
