extern crate rodio;
extern crate piston_window;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Seek};
use std::path::Path;
use std::thread;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Sink};

fn main() -> Result<(), Box<dyn Error>> {
    // Construct the path to the audio file
    let path = Path::new("res").join("background.wav");

    // Open the audio file and create a decoder for it
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let source = Decoder::new(&mut reader)?;

    // Create an output stream and sink to play the audio
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.append(source);

    // Enable continuous loop
    sink.set_looping(true);

    // Play the audio
    sink.play();

    // Wait for a bit
    thread::sleep(Duration::from_secs(10));

    // Disable continuous loop
    sink.set_looping(false);

    // Wait for a bit
    thread::sleep(Duration::from_secs(2));

    // Pause the audio
    sink.pause();

    // Wait for a bit
    thread::sleep(Duration::from_secs(2));

    // Resume the audio
    sink.play();

    // Wait for the audio to finish playing
    thread::sleep(Duration::from_secs(10));

    Ok(())
}

