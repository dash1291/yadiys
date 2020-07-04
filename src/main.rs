pub mod oscillator;
pub mod synth;

use crate::oscillator::Oscillator;
use crate::synth::Synth;
use std::time::Duration;

use std::thread;

extern crate portaudio_rs as portaudio;
extern crate portmidi as pm;
use portaudio::{stream, hostapi, device};


static SECONDS: usize = 1;

fn main()
{
    portaudio::initialize().unwrap();
    print_devs();
    println!("{:?}", demo());
    portaudio::terminate().unwrap();
}

fn print_devs()
{
    for i in 0 .. portaudio::device::get_count().unwrap()
    {
        match portaudio::device::get_info(i)
        {
            None => {},
            Some(info) => println!("{}: {}", i, info.name),
        }
    }
}

fn demo() -> portaudio::PaResult
{

    let mut phase = 0.0f32;
    let mut buffer = Vec::with_capacity(44100 * SECONDS);
    buffer = vec![0.; 44100*SECONDS];
    let mut synth = Synth::new();
    let mut synth2 = Synth::new();

    let context = pm::PortMidi::new().unwrap();

    // get the device info for the given id
    let info = context.device(0).unwrap();
    println!("Listening on: {}) {}", info.id(), info.name());

    let in_port = context.input_port(info, 1024).unwrap();

    let callback = Box::new(move |_input: &[f32], output: &mut [f32], _time: stream::StreamTimeInfo, _flags: stream::StreamCallbackFlags| -> stream::StreamCallbackResult
        {
            if let Ok(Some(event)) = in_port.read_n(1024) {
                println!("{:?}", event[0].message);
                if event[0].message.status == 144 {
                    println!("{:?}", event[0].message.data1);
                    synth.trigger_attack(event[0].message.data1);
                } else {
                    synth.trigger_release(event[0].message.data1);
                }
            }
            // there is no blocking receive method in PortMidi, therefores
            // we have to sleep some time to prevent a busy-wait loop
            
            synth.output(output, output.len());
    
            stream::StreamCallbackResult::Continue
        });

        let stream = portaudio::stream::Stream::open_default(0, 2, 44100.0, portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED, Some(callback))?;

        stream.start()?;
        loop {
            thread::sleep(Duration::from_secs(10));
        }
/*
    let waiter = std::thread::spawn(move|| {
        std::thread::sleep(std::time::Duration::from_secs(SECONDS as u64));
    });



    let _ = waiter.join();
*/
    Ok(())
}
