pub mod oscillator;
pub mod synth;

use crate::oscillator::Oscillator;
use crate::synth::Synth;

extern crate portaudio_rs as portaudio;

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
    let stream = portaudio::stream::Stream::open_default(0, 1, 44100.0, portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED, None)?;

    stream.start()?;

    let mut phase = 0.0f32;
    let mut buffer = Vec::with_capacity(44100 * SECONDS);
    buffer = vec![0.; 44100*SECONDS];
    let mut synth = Synth::new();

    synth.play_note(69);
    synth.output(&mut buffer, 44100 * SECONDS);


    let waiter = std::thread::spawn(move|| {
        std::thread::sleep(std::time::Duration::from_secs(SECONDS as u64));
    });

    match stream.write(&*buffer)
    {
        Err(e) => { println!("write 1: Err({:?})", e); },
        Ok(()) => {},
    }

    let _ = waiter.join();

    Ok(())
}