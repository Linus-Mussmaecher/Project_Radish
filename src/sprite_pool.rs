use regex;
use std::{collections::HashMap, path::Path, time::Duration};

use ggez::*;
use mooeye::sprite::Sprite;

/// A pool that contains a number of initialized [sprite::Sprite]s at once and can be passed around and allows initialization of sprites using the prototype pattern and without having to re-access the file system or pass around a loading context.
/// Provides functions for quickly initalizing folders of sprites and access methods similar to those of [graphics::Image] and [mooeye::sprite::Sprite].
pub struct SpritePool {
    sprites: HashMap<String, Sprite>,
}

impl SpritePool {
    /// Creates a new (empty) [SpritePool] instance.
    pub fn new() -> Self {
        Self {
            sprites: HashMap::new(),
        }
    }

    /// Loads all sprites within the given folder (relative to the ggez resource directory, see [ggez::context::ContextBuilder]) into the sprite pool.
    /// Sprite names need to be formatted as described in [sprite::Sprite], only .png and .jpeg/.jpg images will be loaded.
    pub fn with_folder(mut self, ctx: &Context, path: impl AsRef<Path>) -> Self {
        let paths = ctx
            .fs
            .read_dir(path)
            .expect("Could not find specified path.");

        let sprite_match = regex::Regex::new(r".*_\d*_\d*.[png|jpg|jpeg]").unwrap();

        for path in paths {
            let path_string = path.to_string_lossy().to_string();
            if sprite_match.is_match(&path_string){
                if let Ok(sprite) = Sprite::from_path_fmt(path.clone(), ctx, Duration::from_secs_f32(0.25)) {
                    self.sprites.insert(
                        path_string.replace("\\", "/"),
                        sprite,
                    );
                }
            }
        }
        self
    }

    /// Loads all sprites within the given folder (relative to the ggez resource directory, see [ggez::context::ContextBuilder]) and all its subfolder into the sprite pool.
    /// Sprite names need to be formatted as described in [sprite::Sprite], only .png and .jpeg images will be loaded.
    pub fn with_folder_rec(mut self, ctx: &Context, path: impl AsRef<Path>) -> Self {
        let paths = ctx
            .fs
            .read_dir(path)
            .expect("Could not find specified path.");

        let sprite_match = regex::Regex::new(r".*_\d*_\d*.[png|jpg|jpeg]").unwrap();

        for path in paths {
            let path_string = path.to_string_lossy().to_string();
            if sprite_match.is_match(&path_string){
                if let Ok(sprite) = Sprite::from_path_fmt(path.clone(), ctx, Duration::from_secs_f32(0.25)) {
                    self.sprites.insert(
                        path_string.replace("\\", "/"),
                        sprite,
                    );
                }
            }else{
                self = self.with_folder_rec(ctx, path);
            }
        }
        println!("Now containing {} files.", self.sprites.len());
        self
    }

    /// Initialies a sprite from the sprite pool.
    /// The path syntax is exactly the same as for initalizing images or sprites, relative to the ggez resource folder.
    /// See [graphics::Image] and [sprite::Sprite].
    /// If the sprite (path) is not yet contained in the pool, an error is returned.
    /// For lazy initalization, use [init_sprite_lazy] instead.
    pub fn init_sprite(
        &self,
        path: impl AsRef<Path>,
        frame_time: Duration,
    ) -> Result<Sprite, GameError> {
        let sprite = self
            .sprites
            .get(&path.as_ref().to_string_lossy().to_string())
            .ok_or_else(|| GameError::CustomError("Could not find sprite.".to_owned()))?;
        Ok((*sprite).clone())
    }

    /// Initialies a sprite from the sprite pool.
    /// The path syntax is exactly the same as for initalizing images or sprites, relative to the ggez resource folder.
    /// See [graphics::Image] and [sprite::Sprite].
    /// If the sprite (path) is not yet contained in the pool, the system will attempt to load it from the file system and return it.
    /// If this also fails, an error is returned.
    pub fn init_sprite_lazy(
        &mut self,
        ctx: &Context,
        path: impl AsRef<Path>,
        frame_time: Duration,
    ) -> Result<Sprite, GameError> {
        let key = &path.as_ref().to_string_lossy().to_string();
        if !self.sprites.contains_key(key) {
            let sprite = Sprite::from_path_fmt(path, ctx, frame_time)?;
            self.sprites.insert((*key).clone(), sprite);
        }
        let sprite = self.sprites.get(key).ok_or_else(|| {
            GameError::CustomError(
                "Could not find sprite even after error-free load attempt.".to_owned(),
            )
        })?;
        Ok((*sprite).clone())
    }
}
