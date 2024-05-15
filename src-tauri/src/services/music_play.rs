use std::{fs::File, io::BufReader, sync::Arc};
use tokio::sync::{broadcast, Mutex};

use rodio::{ Decoder, OutputStream, Sink};
use tokio::sync::broadcast::Sender;


#[derive(Debug, Clone)]
pub enum MusicEvent {
    Play(String),
    Recovery,
    Pause,
    Volume(f32),
}

/// Music processing services. The main job is to receive [`MusicEvent`] and process them
pub struct MusicService {
    pub event_sender: Sender<MusicEvent>,
    _stream: OutputStream, // sink need the stream, ensuring that their lifecycles are the same
    pub sink: Arc<Mutex<Sink>>,
}

impl MusicService {
    pub fn new() -> Self {
        // Create a tokio broadcast channel to transmit events
        let (event_sender, mut event_receiver) = broadcast::channel(100);
        // Create a Rodio sink and use Arc and Mutex to share data. If not. The ownership of the sink will be Moved and the sink will not be able to be used in the future.
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Arc::new(Mutex::new(Sink::try_new(&handle).unwrap()));
        let sink_clone = Arc::clone(&sink);

        tokio::spawn(async move {
            while let Ok(event) = event_receiver.recv().await {
                let sink = sink_clone.lock().await;
                match event {
                    MusicEvent::Play(file_path) => {
                        let file = BufReader::new(File::open(file_path).unwrap());
                        let source = Decoder::new(file).unwrap();
                        sink.clear();
                        if sink.is_paused() {
                            sink.append(source);
                            sink.play();
                        }
                    }
                    MusicEvent::Pause => {
                        sink.pause();
                    }
                    MusicEvent::Volume(volume) => {
                        sink.set_volume(volume / 50.0);
                    }
                    MusicEvent::Recovery => {
                        sink.play();
                    }
                }
            }
        });

        Self {
            event_sender,
            _stream,
            sink,
        }
    }
}

impl Default for MusicService {
    fn default() -> Self {
        Self::new()
    }
}