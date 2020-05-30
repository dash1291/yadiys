pub mod oscillator;
pub mod synth;

use crate::oscillator::Oscillator;
use crate::synth::Synth;
extern crate portaudio;
extern crate portmidi as pm;

use portaudio as pa;
use std::f64::consts::PI;

const CHANNELS: i32 = 1;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

fn run() -> Result<(), pa::Error> {
    println!(
        "PortAudio Test: output sine wave. SR = {}, BufSize = {}",
        SAMPLE_RATE, FRAMES_PER_BUFFER
    );

    let pa = pa::PortAudio::new()?;

    let mut phase = 0.;
    let frequency = 440.;
    let mut settings =
        pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;

    // we won't output out of range samples so don't bother clipping them.
    // settings.flags = pa::stream_flags::CLIP_OFF;

    // This routine will be called by the PortAudio engine when audio is needed. It may called at
    // interrupt level on some machines so don't do anything that could mess up the system like
    // dynamic resource allocation or IO.

    let mut synth = Synth::new();
    let mut synth2 = Synth::new();

    let context = pm::PortMidi::new().unwrap();

    // get the device info for the given id
    let info = context.device(0).unwrap();
    println!("Listening on: {}) {}", info.id(), info.name());

    // get the device's input port
    let in_port = context.input_port(info, 1024).unwrap();

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        if let Ok(Some(event)) = in_port.read_n(1024) {
            synth.play_note(event[0].message.data1);
        }
        synth.output(buffer, frames);
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    println!("Play for {} seconds.", NUM_SECONDS);
    pa.sleep(NUM_SECONDS * 1_000);

    stream.stop()?;
    stream.close()?;

    println!("Test finished.");

    Ok(())
}
