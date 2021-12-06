use std::sync::{Arc, Mutex};

use byte_slice_cast::AsByteSlice;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample,
};
use fluidlite::{Settings, Synth};
#[macro_use]
extern crate lazy_static;
#[derive(Debug)]
struct Opt {
    #[cfg(all(
        any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
        feature = "jack"
    ))]
    jack: bool,

    device: String,
}

lazy_static! {
    static ref buffer: [i16; 44100 * 2] = [0i16; 44100 * 2];
}

impl Opt {
    fn from_args() -> Self {
        let app = clap::App::new("beep").arg_from_usage("[DEVICE] 'The audio device to use'");
        #[cfg(all(
            any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
            feature = "jack"
        ))]
        let app = app.arg_from_usage("-j, --jack 'Use the JACK host");
        let matches = app.get_matches();
        let device = matches.value_of("DEVICE").unwrap_or("default").to_string();

        #[cfg(all(
            any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
            feature = "jack"
        ))]
        return Opt {
            jack: matches.is_present("jack"),
            device,
        };

        #[cfg(any(
            not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),
            not(feature = "jack")
        ))]
        Opt { device }
    }
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    // Conditionally compile with jack if the feature is specified.
    #[cfg(all(
        any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"),
        feature = "jack"
    ))]
    // Manually check for flags. Can be passed through cargo with -- e.g.
    // cargo run --release --example beep --features jack -- --jack
    let host = if opt.jack {
        cpal::host_from_id(cpal::available_hosts()
            .into_iter()
            .find(|id| *id == cpal::HostId::Jack)
            .expect(
                "make sure --features jack is specified. only works on OSes where jack is available",
            )).expect("jack host unavailable")
    } else {
        cpal::default_host()
    };

    #[cfg(any(
        not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd")),
        not(feature = "jack")
    ))]
    let host = cpal::default_host();

    let device = if opt.device == "default" {
        host.default_output_device()
    } else {
        host.output_devices()?
            .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
    }
    .expect("failed to find output device");
    println!("Output device: {}", device.name()?);

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    let settings = Settings::new().unwrap();

    let synth = Synth::new(settings).unwrap();
    synth.sfload("Nintendo_64_ver_3.0.sf2", true).unwrap();

    //static buffer: Arc<Mutex<[i16; 44100 * 2]>> = Arc::new(Mutex::new([0i16; 44100 * 2]));
    //let mut file = File::create("soundfont-sample.pcm").unwrap();
    synth.note_on(0, 10, 127).unwrap();
    synth.write(buffer.as_mut()).unwrap();

    //file.write(buffer.as_byte_slice()).unwrap();

    synth.note_on(0, 50, 127).unwrap();
    synth.write(buffer.as_mut()).unwrap();
    //file.write(buffer.as_byte_slice()).unwrap();

    synth.note_off(0, 10).unwrap();
    synth.write(buffer.as_mut()).unwrap();

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), buffer),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), buffer),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), buffer),
    }
}

pub fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    bufferIn: &'static [i16],
) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    // let mut next_value = move || {
    //     sample_clock = (sample_clock + 1.0) % sample_rate;
    //     //println!("sample {}", sample_clock);
    //     (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    // };

    println!("size {}", bufferIn.len());
    // for b in buffer.as_byte_slice() {
    //     println!("buffer {}", b);
    // }

    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        //println!("ready");
        //let b = buffer.as_chunks()
        // println!("byte {}", b.len());
        //let f = [(sample_clock) as usize] as f32;
        //println!("f {}", f);
        ((bufferIn[(2 * sample_clock as usize)]) as f32) / 128.
        //(sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(200));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
