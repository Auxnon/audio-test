// use byte_slice_cast::AsByteSlice;
// use fluidlite::{Settings, Synth};
// use std::{fs::File, io::Write};
// fn main() {
//     let settings = Settings::new().unwrap();

//     let synth = Synth::new(settings).unwrap();
//     synth.sfload("Nintendo_64_ver_3.0.sf2", true).unwrap();

//     let mut buffer = [0i16; 44100 * 2];
//     let mut file = File::create("soundfont-sample.pcm").unwrap();

//     synth.note_on(0, 60, 127).unwrap();
//     synth.write(buffer.as_mut()).unwrap();
//     file.write(buffer.as_byte_slice()).unwrap();

//     synth.note_on(0, 50, 127).unwrap();
//     synth.write(buffer.as_mut()).unwrap();
//     file.write(buffer.as_byte_slice()).unwrap();

//     synth.note_off(0, 60).unwrap();
//     synth.write(buffer.as_mut()).unwrap();
//     file.write(buffer.as_byte_slice()).unwrap();
// }

use byte_slice_cast::AsByteSlice;
use fluidlite::{Settings, Synth};
use rodio::buffer::SamplesBuffer;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::{fs::File, io::Write};
fn main() {
    let settings = Settings::new().unwrap();

    let synth = Synth::new(settings).unwrap();
    synth.sfload("Nintendo_64_ver_3.0.sf2", true).unwrap();

    let mut buffer = [0i16; 44100 * 2];
    //let mut file = File::create("soundfont-sample.pcm").unwrap();

    synth.note_on(0, 60, 127).unwrap();
    synth.write(buffer.as_mut()).unwrap();

    //file.write(buffer.as_byte_slice()).unwrap();

    synth.note_on(0, 50, 127).unwrap();
    synth.write(buffer.as_mut()).unwrap();
    //file.write(buffer.as_byte_slice()).unwrap();

    synth.note_off(0, 60).unwrap();
    synth.write(buffer.as_mut()).unwrap();
    //file.write(buffer.as_byte_slice()).unwrap();

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("examples/music.ogg").unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    //stream_handle.play_raw(source.convert_samples());

    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    let read=rodio::Decoder::new(&mut buffer.as_byte_slice());
    let source = rodio::Decoder::new(read).unwrap();

    sink.append(Source::); //BufReader::new(file)

    sink.sleep_until_end();

    //let sample = SamplesBuffer::new(1, 44100, buffer);
    //stream_handle.play_raw::<[i16; 44100 * 2]>(buffer);

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    //std::thread::sleep(std::time::Duration::from_secs(5));
}

/*
use alto::{Alto, AltoResult, Mono, Source};
use std::{thread, time};

fn main() {
    use std::process::exit;
    if let Err(e) = run() {
        println!("Failed to run basic example: {}", e);
        exit(1);
    }
}

fn run() -> AltoResult<()> {
    use std::sync::Arc;

    let result = Alto::load_default()?;

    println!("here");
    for s in alto?.enumerate_outputs() {
        println!("Found device: {}", s.to_str().unwrap());
    }

    let device = alto.open(None)?; // Opens the default audio device
    let context = device.new_context(None)?; // Creates a default context

    // Configure listener
    context.set_position([1.0, 4.0, 5.0])?;
    context.set_velocity([2.5, 0.0, 0.0])?;
    context.set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))?;

    let mut _source = context.new_static_source()?;

    // Now you can load your samples and store them in a buffer with
    // `context.new_buffer(samples, frequency)`;

    let pi = std::f32::consts::PI;
    let data: Vec<_> = (0..88200u32)
        .map(|i| ((i16::MAX as f32) * f32::sin(2.0 * pi * (i as f32) * 220.0 / 44100.0)) as i16)
        .collect();
    let buffer = context.new_buffer::<Mono<i16>, _>(data, 44_100);
    let buf = Arc::new(buffer.unwrap());

    let good_result = _source.set_buffer(buf);
    assert!(good_result.is_ok() && !good_result.is_err());

    _source.play();

    thread::sleep(time::Duration::from_millis(2000));
    Ok(())
}
*/