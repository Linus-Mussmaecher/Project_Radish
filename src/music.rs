use std::collections::VecDeque;

use good_web_game::{audio, *};

pub struct MusicPlayer {
    volume: f32,
    current_song: Option<audio::Source>,
    playlist: VecDeque<audio::Source>,
    wave: u32,
}

impl MusicPlayer {
    #[allow(dead_code)]
    /// Creates a new MusicPlayer with the specified playlist
    pub fn new(playlist: VecDeque<audio::Source>) -> Self {
        Self {
            volume: 0.01,
            current_song: None,
            playlist,
            wave: 0,
        }
    }

    /// Creates a new MusicPlayer with all .wav or .mp3 files from the selected folder.
    pub fn from_folder(ctx: &mut Context, path: impl AsRef<std::path::Path>) -> Self {
        let mut playlist = VecDeque::new();
        // let paths = ctx
        //     .fs
        //     .read_dir(path.as_ref())
        //     .expect("Could not find specified path.");

        // for sub_path in paths {
        //     let path_string = sub_path.to_string_lossy().to_string();
        //     let len = path_string.len();
        //     if path_string[len - 4..] == *".wav" || path_string[len - 4..] == *".mp3" {
        //         if let Ok(source) = audio::Source::new(ctx, sub_path) {
        //             playlist.push_back(source);
        //         }
        //     }
        // }
        // TODO: Song loading.

        Self {
            volume: 0.5,
            current_song: None,
            playlist,
            wave: 0,
        }
    }

    /// Checks if the currently playing song is finished and starts the next one if neccessary
    /// Also corrects the song volume.
    pub fn check_song(&mut self, ctx: &mut Context, wave: u32) {
        if let Some(song) = &mut self.current_song {
            if song.volume() != self.volume {
                song.set_volume(ctx, self.volume);
            }
            if self.wave != wave {
                self.wave = wave;
                self.next_song(ctx);
            }
        }
    }

    /// Stops the currently playing song and starts the next one from the list.
    pub fn next_song(&mut self, ctx: &mut Context) {
        self.stop(ctx);
        self.current_song = self.playlist.pop_front();
        if let Some(song) = &mut self.current_song {
            song.set_repeat(true);
            song.set_volume(ctx, self.volume);
            song.play(ctx).unwrap();
        }
    }

    /// Stops the currently playing song (if there is one playing) and puts it back into the queue.
    pub fn stop(&mut self, ctx: &mut Context) {
        if let Some(mut song) = self.current_song.take() {
            song.stop(ctx).unwrap();
            self.playlist.push_back(song);
        }
    }

    /// Checks for changes in the options file to change music volume if neccessary.
    pub fn poll_options(&mut self) {
        self.volume = crate::options::OPTIONS
            .with(|opt| *opt.borrow())
            .music_volume as f32
            / 100.
            * 0.15;
    }
}
