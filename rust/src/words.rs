use crate::letters::Letter;

use godot::{
    engine::{AudioStream, Texture2D},
    prelude::*,
};
use serde::Deserialize;

use std::{collections::HashMap, sync::LazyLock};

pub static WORDS: LazyLock<Words> =
    LazyLock::new(|| toml::from_str(include_str!("../assets/words.toml")).unwrap());

pub struct FoundWord {
    word: GString,
    description: GString,
    picture: Gd<Texture2D>,
    audio: Gd<AudioStream>,
}

#[derive(Deserialize)]
pub struct Words(HashMap<String, WordMeta>);
impl Words {
    /// Constructs a found word from a slice of letters, if it exists, to be
    /// used in a scene.
    pub fn get(&self, word: &[Letter]) -> Option<FoundWord> {
        let word = Self::make_string_from_word(word);
        let WordMeta {
            picture,
            audio,
            description,
        } = self.0.get(&word)?;

        let word = word.into();
        let description = description.into();
        let picture = try_load(picture)
            .inspect_err(|e| godot_error!("{e}"))
            .ok()?;
        let audio = try_load(audio).inspect_err(|e| godot_error!("{e}")).ok()?;

        Some(FoundWord {
            word,
            description,
            picture,
            audio,
        })
    }

    /// Safety: assumes that Letter is a u8 index of ascii letters. If for some
    /// reason this would change, say, if we supported other languages, this
    /// would be very unsafe and could contain null bytes or worse.
    fn make_string_from_word(word: &[Letter]) -> String {
        unsafe {
            String::from_utf8_unchecked(word.iter().map(|letter| *letter as u8 + b'a').collect())
        }
    }
}

/// Note that we use StringName here because we can't use GString directly, and
/// we want the performance benefit of already having them in a Godot format.
#[derive(Deserialize)]
struct WordMeta {
    description: StringName,
    picture: StringName,
    audio: StringName,
}
