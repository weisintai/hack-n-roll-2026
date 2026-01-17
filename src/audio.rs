use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct AudioPlayer {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Option<Sink>,
}

impl AudioPlayer {
    pub fn new() -> Option<Self> {
        match OutputStream::try_default() {
            Ok((stream, stream_handle)) => Some(Self {
                _stream: stream,
                _stream_handle: stream_handle,
                sink: None,
            }),
            Err(_) => {
                // Audio not available, continue silently
                None
            }
        }
    }

    fn play_file(&mut self, filename: &str, should_loop: bool) {
        // Refresh audio stream
        if let Ok((stream, stream_handle)) = OutputStream::try_default() {
            self._stream = stream;
            self._stream_handle = stream_handle;
        }
        
        // Try to find the audio file in common locations
        let possible_paths = [
            format!("assets/{}", filename),
            filename.to_string(),
            format!("../assets/{}", filename),
        ];

        for path in &possible_paths {
            if Path::new(path).exists() {
                if let Ok(file) = File::open(path) {
                    let reader = BufReader::new(file);
                    if let Ok(source) = Decoder::new(reader) {
                        if let Ok(sink) = Sink::try_new(&self._stream_handle) {
                            if should_loop {
                                sink.append(source.repeat_infinite());
                            } else {
                                sink.append(source);
                            }
                            sink.play();
                            self.sink = Some(sink);
                            return;
                        }
                    }
                }
            }
        }
    }

    /// Play the start sound effect (when countdown begins)
    pub fn play_start_sfx(&mut self) {
        self.stop(); // Stop any currently playing audio
        self.play_file("start.mp3", true); // Loop until stopped
    }

    /// Play the end sound effect (when translation completes)
    pub fn play_end_sfx(&mut self) {
        self.stop(); // Stop the start sound
        self.play_file("end.mp3", false); // Play once
    }

    /// Stop the currently playing sound
    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
    }

    /// Check if audio is currently playing
    pub fn is_playing(&self) -> bool {
        self.sink.as_ref().map(|s| !s.empty()).unwrap_or(false)
    }
}
