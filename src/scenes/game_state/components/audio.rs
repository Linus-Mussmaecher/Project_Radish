use ggez::{
    audio::{self, SoundSource},
    glam::Vec2,
    mint::Point3,
};
use legion::{IntoQuery, World};

/// The audio component manages the sounds played by an entity whenever a certain actions is triggered.
pub struct Audio {
    /// Alist of actions and sound.
    /// Whenever the entity triggers an actions, the sounds of any pairs whose first component is of the same action type are played.
    sounds: Vec<(super::actions::GameAction, audio::SpatialSource)>,
}

impl Audio {
    /// Creates a new audio component from a list of action/sound pairs
    pub fn new(sounds: Vec<(super::actions::GameAction, audio::SpatialSource)>) -> Self {
        Self { sounds }
    }

    /// Creates a new audio component with a single action/sound pair.
    pub fn new_single(action: super::actions::GameAction, sound: audio::SpatialSource) -> Self {
        Self {
            sounds: vec![(action, sound)],
        }
    }
}

/// The system that handles the playing of audio effects from entitites.
pub fn audio_system(ctx: &ggez::Context, player_pos: Vec2, world: &mut World) {
    // go over all entities that have actions, a position and a sound component
    for (audio, pos, actions) in
        <(&mut Audio, &super::Position, &super::Actions)>::query().iter_mut(world)
    {
        // check all actions they are doing this frame
        for action2 in actions.get_actions() {
            // check all their sound/action pairs
            for (action1, sound) in audio.sounds.iter_mut() {
                // if the enum-type matches
                if std::mem::discriminant(action1) == std::mem::discriminant(action2) {
                    // set the positions
                    sound.set_position(Point3 {
                        x: pos.x,
                        y: pos.y,
                        z: 0.,
                    });
                    sound.set_ears(
                        Point3 {
                            x: player_pos.x,
                            y: player_pos.y,
                            z: 0.,
                        },
                        Point3 {
                            x: player_pos.x,
                            y: player_pos.y,
                            z: 0.,
                        },
                    );
                    // play the sound
                    match sound.play_detached(ctx) {
                        Ok(_) => {}
                        Err(err) => println!("[ERROR] Error playing sound: {}.", err),
                    };
                }
            }
        }
    }
}
