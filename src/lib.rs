use rodio::{Decoder, Sink};
use std::fs::{self, File};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum AppError {
    SongNotFound(String),
    NetworkError(String),
    PlaybackError(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::SongNotFound(s) => write!(f, "Song not found: {}", s),
            AppError::NetworkError(s) => write!(f, "NetworkError: {}", s),
            AppError::PlaybackError(s) => write!(f, "PlaybackError: {}", s),
            AppError::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IoError(e)
    }
}

pub struct MusicPlayer {
    pub song_list : Vec<PathBuf>,
    pub current_song : Option<String>,
    pub network : Arc<Mutex<bool>>,
    sink : Sink,
    _stream : rodio::OutputStream,
}

impl MusicPlayer {
    pub fn new() -> Result<Self, AppError> {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .map_err(|e| AppError::PlaybackError(e.to_string()))?;
        let sink = Sink::connect_new(stream.mixer());
            
        Ok(MusicPlayer {
            song_list: Vec::new(),
            current_song: None,
            network: Arc::new(Mutex::new(true)),
            sink,
            _stream : stream,
        })
    }
    fn check_connection(&self) -> Result<(), AppError> {
        let connected = self.network.lock().unwrap();
        if *connected {
            Ok(())
            } else {
                Err(AppError::NetworkError("No Network connnection".to_string()))
        }
    }
    pub fn simulate_network(&self, connected: bool) {
        let mut state = self.network.lock().unwrap();
        *state = connected;
        if connected {
            println!("Network: connected");
        } else {
            println!("network: disconnected");
        }
    }
    pub fn song_list(&mut self) -> Result<&Vec<PathBuf>, AppError>{
        self.check_connection()?;

        self.song_list = fs::read_dir("./music")?
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                match path.extension()?.to_str()? {
                    "mp3" => Some(path),
                    _ => None,
                }
            })
            .collect();
        Ok(&self.song_list)
    }
    pub fn find_song(&self, name: &str) -> Option<&PathBuf>{
        self.song_list.iter().find(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.contains(name))
                .unwrap_or(false)
        })
    }
    pub fn play_song(&mut self, path: &PathBuf) -> Result<(), AppError> {
        self.check_connection()?;

        let song_file = File::open(path)?;
        let source = Decoder::try_from(song_file)
            .map_err(|e| AppError::PlaybackError(e.to_string()))?;
        self.sink.stop();
        self.sink.append(source);
        self.sink.play();

        self.current_song = path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.to_string());
        println!("now playing: {}", self.current_song.as_deref().unwrap_or("unkown"));
            Ok(())
    }
    pub fn pause(&self) {
        self.sink.pause();
        println!("Paused");
    }
    pub fn resume(&self) {
        self.sink.play();
        println!("Resumed");
    }
    pub fn stop(&self) {
        self.sink.stop();
        self.current_song.as_deref().map(|s| println!("Stopped: {}", s));
        println!("Stopped playing");
    }
}

