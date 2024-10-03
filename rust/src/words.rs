use godot::{
    engine::{AudioStream, AudioStreamRandomizer, Label, Texture2D, TextureRect},
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
        let word_meta: Gd<ExtWordMeta> = self.base().get_node_as("ExtWordMeta");
        let mut word: Gd<Label> = self.base().get_node_as("Word");
        let mut picture: Gd<TextureRect> = self.base().get_node_as("Picture");
        let mut description: Gd<Label> = self.base().get_node_as("Description");
        let mut audio: Gd<AudioStreamRandomizer> = self
            .base()
            .get_node_as::<AudioStreamPlayer>("Audio")
            .get_stream()
            .unwrap()
            .cast();

        word.set_text(word_meta.bind().get_word());
        picture.set_texture(word_meta.bind().get_picture());
        description.set_text(word_meta.bind().get_description());
        audio.add_stream(-1, word_meta.bind().get_audio());
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
pub struct Words(HashMap<StringName, StringName>);
impl Words {
    /// Constructs a word's metadata from a StringName, if it exists, to be
    /// used in an `ExtShowWord` scene.
    pub fn get(&self, word: StringName) -> Option<Gd<ExtWordMeta>> {
        let description = self.0.get(&word)?;

        let word = word.into();
        let description = description.into();
        let picture = load(format!("res://assets/licensed/pixabay/words/{}.webp", word));
        let audio = load(format!("res://assets/licensed/luvvoice/words/{}.mp3", word));

        let mut word_meta = Gd::from_init_fn(|base| ExtWordMeta {
            base,

            word,
            description,
            picture,
            audio,
        });
        word_meta.set_name("ExtWordMeta".into());

        Some(word_meta)
    }
}
