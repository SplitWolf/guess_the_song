//IDEA: Provide a youtube/spotify playlist
// using the playlist select a random song
// after select a random start time in the (first half)? of the song
// Play the song clip, while allowing gueses, then lock guesses
// Reveal Song/Maybe video
// Repeat for number of rounds
// Druid or native-windows-gui for GUI, maybe Iced

use std::fs::File;
use std::io::BufReader;
//use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
//use rodio::source::{SineWave, Source};


fn main() {
  
  
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("examples/river.mp3").unwrap());
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let source =  Decoder::new(file).unwrap();
    // Add a dummy source of the sake of the example.
    //let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
    sink.append(source);
    sink.set_volume(0.2);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    //sink.sleep_until_end();
  //  println!("Hello, world!");
}
