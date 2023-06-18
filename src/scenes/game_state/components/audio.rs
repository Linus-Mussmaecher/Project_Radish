use std::collections::HashMap;
use crate::scenes::options;
use ggez::audio::{self, SoundSource};
use legion::system;

#[system(for_each)]
pub fn audio_enqueue(actions: &super::Actions, #[resource] audio_pool: &mut AudioPool) {
    for action in actions.get_actions() {
        if let super::actions::GameAction::PlaySound(sound) = action {
            audio_pool.sound_queue.push(sound.clone());
        }
    }
}

/// The system that handles the playing of audio effects from entitites.
pub fn audio_play_system(
    ctx: &ggez::Context,
    res: &mut legion::Resources,
) -> Result<(), ggez::GameError> {
    // get boundaries for relative moving
    let audio_pool = &mut *res
        .get_mut::<AudioPool>()
        .ok_or_else(|| ggez::GameError::CustomError("Could not unpack audio pool.".to_owned()))?;

    for sound in audio_pool.sound_queue.iter() {
        // play the sound
        match audio_pool.sources.get_mut(sound) {
            Some(sound) => {
                sound.set_volume(audio_pool.options.volume as f32 / 100.);
                sound.play_detached(ctx)?;
            }
            None => {}
        };
    }
    audio_pool.sound_queue.clear();

    Ok(())
}

/// A pool that contains a number of initialized [ggez::audio::Source]s at once and can be passed around and allows playing audio sources while only saving keys.
pub struct AudioPool {
    /// The pooled sources.
    sources: HashMap<String, audio::Source>,
    /// the queued sounds
    sound_queue: Vec<String>,
    /// an options struct to customize volumes
    options: options::OptionsConfig,
}

impl AudioPool {
    /// Creates a new (empty) [AudioPool] instance.
    pub fn new(options: options::OptionsConfig) -> Self {
        Self {
            sources: HashMap::new(),
            sound_queue: Vec::new(),
            options,
        }
    }

    /// Loads all sources within the given folder (relative to the ggez resource directory, see [ggez::context::ContextBuilder]) into the audio pool.
    /// Can also search all subfolders.
    pub fn with_folder(
        mut self,
        ctx: &ggez::Context,
        path: impl AsRef<std::path::Path>,
        search_subfolders: bool,
    ) -> Self {
        let paths = ctx
            .fs
            .read_dir(path.as_ref())
            .expect("Could not find specified path.");

        for sub_path in paths {
            let path_string = sub_path.to_string_lossy().to_string();
            let len = path_string.len();
            if path_string[len - 4..] == *".wav" || path_string[len -4..] == *".ogg" {
                if let Ok(source) = audio::Source::new(ctx, sub_path) {
                    self.sources
                        .insert(path_string.replace(r"\", "/")[..len - 4].to_owned(), source);
                }
            } else if search_subfolders {
                self = self.with_folder(ctx, sub_path, search_subfolders);
            }
        }
        //println!("Now containing {} files.", self.sources.len());
        self
    }
}
