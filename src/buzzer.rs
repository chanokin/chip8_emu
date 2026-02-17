use rodio;
use rodio::source::{SineWave, Source};
use std::time::Duration;
use crate::errors::BuzzerError;

pub struct Buzzer {
    is_on: bool,
    stream_handle: rodio::OutputStream,
    sink: rodio::Sink,
    volume: f32,
    frequency: f32,
}

impl Buzzer {
    pub fn new(volume: f32, frequency: f32) -> Self {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("No audio device found");
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        Buzzer {
            is_on: false,
            stream_handle,
            sink,
            volume,
            frequency
        }
    }

    pub fn turn_on(&mut self) -> Result<(), BuzzerError> {
        if self.is_on {
            return Ok(()); // Already on, do nothing
        }
        if self.sink.empty() {
            let source = SineWave::new(self.frequency)
                            .take_duration(Duration::from_secs_f32(3f32/self.frequency)) // Play for 1 second, then stop
                            .repeat_infinite()
                            .amplify(self.volume);

            self.sink.append(source);
            self.is_on = true;
        }

        self.sink.play();

        Ok(())
    }

    pub fn turn_off(&mut self) -> Result<(), BuzzerError> {
        if self.is_on {
            self.sink.pause();
            self.is_on = false;
        }
        Ok(())
    }
}


