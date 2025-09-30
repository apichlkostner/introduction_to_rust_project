//! Sound effect logic for Pong.
//!
//! This module provides a simple sound effect system using the `rodio` crate,
//! allowing the game to play a beep sound on events such as collisions.

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

/// Handles sound effects for the game.
///
/// Manages an audio stream and sink for playing short beep sounds.
pub struct SoundEffect {
    _stream_handle: OutputStream,
    sink: Sink,
}

impl SoundEffect {
    /// Creates a new `SoundEffect` instance and initializes the audio stream and sink.
    ///
    /// # Panics
    ///
    /// Panics if the default audio stream cannot be opened.
    pub fn new() -> Self {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");

        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        SoundEffect {
            _stream_handle: stream_handle,
            sink: sink,
        }
    }

    /// Plays a short beep sound if no other sound is currently playing.
    pub fn beep(&self, freq: f32) {
        if self.sink.empty() {
            let source = SineWave::new(freq)
                .take_duration(Duration::from_secs_f32(0.15))
                .amplify(0.20);
            self.sink.append(source);
        }
    }
}
