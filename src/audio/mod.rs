//! The `audio` module contains the necessary components for playing audio files.
//!
//! It consists of the following sub-modules:
//! - `player`: Defines an `AudioPlayer` struct and related functionality for playing audio files.
//! - `runner`: Implements the main functionality for running the audio playback, including frame
//!   rate control and output.
//! - `utils`: Contains utility functions for working with audio files.
pub mod player;
pub mod runner;
pub mod utils;