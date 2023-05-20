extern crate rodio;
extern crate piston_window;

use std::fs::File;
use std::io::BufReader;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::path::Path;
use rodio::Decoder;
use rodio::Sink;

pub struct Audio {
    current_frame: Option<u64>,
    sink: Sink,
    status: String,
}

impl Audio {
    pub fn new() -> Result<Self, &'static str> {
        let file_path = Path::new("res/background.wav");
        let file = File::open(&file_path).map_err(|_| "Error: Could not open file.")?;
        let source = Decoder::new(BufReader::new(file)).map_err(|_| "Error: Could not decode file.")?;
        let device = rodio::default_output_device().ok_or("Error: No audio output device found.")?;
        let sink = Sink::new(&device);
        sink.append(source);
        sink.pause();

        Ok(Self {
            current_frame: None,
            sink,
            status: String::from("paused"),
        })
    }

    pub fn play(&mut self) {
        self.sink.play();
        self.status = String::from("play");
    }

    pub fn pause(&mut self) {
        if self.status == "paused" {
            println!("Audio is already paused.");
            return;
        }
        self.current_frame = Some(self.sink.get_position().unwrap().0);
        self.sink.pause();
        self.status = String::from("paused");
    }

    pub fn resume(&mut self) {
        if self.status == "play" {
            println!("Audio is already playing.");
            return;
        }
        if let Some(frame) = self.current_frame {
            self.sink.seek(SeekFrom::Start(frame)).unwrap();
        }
        self.sink.play();
        self.status = String::from("play");
    }
}
