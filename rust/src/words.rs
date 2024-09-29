use crate::letters::Letter;

use godot::{
    engine::{AudioStream, Texture2D},
    prelude::*,
};
use serde::Deserialize;

use std::{collections::HashMap, sync::LazyLock};

pub static WORDS: LazyLock<Words> =
    LazyLock::new(|| toml::from_str(include_str!("../assets/words.toml")).unwrap());

#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub struct ExtShowWord {
    base: Base<Node2D>,
}
#[godot_api]
impl INode2D for ExtShowWord {
    fn ready(&mut self) {
        todo!()
    }
}

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct ExtFoundWord {
    base: Base<Node>,

    #[export]
    word: GString,
    #[export]
    description: GString,
    #[export]
    picture: Gd<Texture2D>,
    #[export]
    audio: Gd<AudioStream>,
}

#[derive(Deserialize)]
pub struct Words(HashMap<String, WordMeta>);
impl Words {
    /// Constructs a found word from a slice of letters, if it exists, to be
    /// used in a scene.
    pub fn get(&self, word: &[Letter]) -> Option<Gd<ExtFoundWord>> {
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

        Some(Gd::from_init_fn(|base| ExtFoundWord {
            base,

            word,
            description,
            picture,
            audio,
        }))
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
