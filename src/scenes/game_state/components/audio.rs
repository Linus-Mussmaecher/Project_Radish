use crate::options;
use good_web_game::audio;
use legion::system;
use std::collections::HashMap;

const SOUNDS_PER_FRAME: usize = 4;

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
    ctx: &mut good_web_game::Context,
    res: &mut legion::Resources,
) -> Result<(), good_web_game::GameError> {
    let audio_pool = &mut *res.get_mut::<AudioPool>().ok_or_else(|| {
        good_web_game::GameError::CustomError("Could not unpack audio pool.".to_owned())
    })?;

    audio_pool.poll_options();

    for sound in audio_pool.sound_queue.iter().take(SOUNDS_PER_FRAME) {
        if !audio_pool.sources.contains_key(sound) {
            audio_pool
                .sources
                .insert(sound.clone(), audio::Source::new(ctx, sound)?);
        }

        // play the sound
        if let Some(sound) = audio_pool.sources.get_mut(sound) {
            sound.set_volume(ctx, audio_pool.options.volume as f32 / 100. * 0.2)?;
            sound.play(ctx)?;
        };
    }
    audio_pool.sound_queue.clear();

    Ok(())
}

/// A pool that contains a number of initialized [good_web_game::audio::Source]s at once and can be passed around and allows playing audio sources while only saving keys.
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

    /// Checks for changes in the options file to change music volume if neccessary.
    pub fn poll_options(&mut self) {
        self.options = crate::options::OPTIONS.with(|opt| *opt.borrow());
    }
}
