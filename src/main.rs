use std::io::Cursor;

use clap::Parser;
use rodio::{Decoder, OutputStream, Source};

#[derive(Parser)]
struct Cli {
    /// Override mp3 file to play when waking up
    #[arg(short, long)]
    mp3_file_path: Option<String>,

    /// Override sleep duration (lower -> you will move more)
    #[arg(short, long, default_value = "30min")]
    sleep_duration: humantime::Duration,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Read mp3 from file or default
    let mut sound = match cli.mp3_file_path {
        Some(path) => Cursor::new(std::fs::read(path)?),
        None => Cursor::new(include_bytes!("samsung-estourado.mp3").to_vec()),
    };

    let sound_duration = mp3_duration::from_read(&mut sound).unwrap();

    // We recreate the output stream everytime we wake up to use less resources while sleeping in
    // exchange for more resources when waking up
    loop {
        {
            // Decode that sound file into a source
            let source = Decoder::new(sound.clone()).unwrap();
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            stream_handle.play_raw(source.convert_samples().amplify(1.5)).unwrap();
            std::thread::sleep(sound_duration);
        }
        std::thread::sleep(cli.sleep_duration.into());
    }
}
