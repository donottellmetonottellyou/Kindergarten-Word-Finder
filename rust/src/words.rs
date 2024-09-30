use godot::{
    engine::{AudioStream, Label, Texture2D, TextureRect},
    prelude::*,
};
use serde::Deserialize;

use std::{collections::HashMap, sync::LazyLock};

/// Singleton of all the words that are in words.toml as `Words`.
pub static WORDS: LazyLock<Words> =
    LazyLock::new(|| toml::from_str(include_str!("../assets/words.toml")).unwrap());

/// Used to load data from its expected child `ExtWordMeta` into its other
/// expected children, Word (Label), Picture (TextureRect), Description
/// (Label), and Audio (AudioStreamPlayer). Should be a root node with Word,
/// Picture, Description and Audio as scene nodes, and `ExtWordMeta` added at
/// runtime.
#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub struct ExtShowWord {
    base: Base<Node2D>,
}
#[godot_api]
impl INode2D for ExtShowWord {
    fn ready(&mut self) {
        let word_meta: Gd<ExtWordMeta> = self
            .base()
            .find_child("ExtFoundWord".into())
            .expect("ExtFoundWord not found")
            .cast();
        let mut word: Gd<Label> = self
            .base()
            .find_child("Word".into())
            .expect("Word Label not found")
            .cast();
        let mut picture: Gd<TextureRect> = self
            .base()
            .find_child("Picture".into())
            .expect("Picture TextureRect not found")
            .cast();
        let mut description: Gd<Label> = self
            .base()
            .find_child("Description".into())
            .expect("Description Label not found")
            .cast();
        let mut audio: Gd<AudioStreamPlayer> = self
            .base()
            .find_child("Audio".into())
            .expect("Audio AudioStreamPlayer not found")
            .cast();

        word.set_text(word_meta.bind().get_word());
        picture.set_texture(word_meta.bind().get_picture());
        description.set_text(word_meta.bind().get_description());
        audio.set_stream(word_meta.bind().get_audio());
        audio.play();
    }
}

/// Represents all metadata to be displayed/played by its intended parent,
/// `ExtShowWord`. Created exclusively from the `WORDS` singleton as a match of
/// a word using `Words.get()`.
#[derive(GodotClass)]
#[class(base=Node, no_init)]
pub struct ExtWordMeta {
    base: Base<Node>,

    #[var(get)]
    word: GString,
    #[var(get)]
    description: GString,
    #[var(get)]
    picture: Gd<Texture2D>,
    #[var(get)]
    audio: Gd<AudioStream>,
}

/// A mapping of words to the metadata to be used by `ExtShowWord`.
#[derive(Deserialize)]
pub struct Words(HashMap<StringName, WordMeta>);
impl Words {
    /// Constructs a word's metadata from a StringName, if it exists, to be
    /// used in an `ExtShowWord` scene.
    pub fn get(&self, word: StringName) -> Option<Gd<ExtWordMeta>> {
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

        Some(Gd::from_init_fn(|base| ExtWordMeta {
            base,

            word,
            description,
            picture,
            audio,
        }))
    }
}

/// Private deserializable metadata representation, to be turned into
/// `ExtWordMeta` at runtime.
#[derive(Deserialize)]
struct WordMeta {
    description: StringName,
    picture: StringName,
    audio: StringName,
}
