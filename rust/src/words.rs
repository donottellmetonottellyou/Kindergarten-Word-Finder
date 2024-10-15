use godot::{
    engine::{
        AudioStream, AudioStreamRandomizer, Label, Texture2D, TextureRect,
    },
    prelude::*,
};
use serde::Deserialize;

use std::{collections::HashMap, ops::Deref, sync::LazyLock};

/// All easy words (words of length 3)
static EASY_WORDS: LazyLock<Words> = LazyLock::new(|| {
    toml::from_str(include_str!("../assets/easy_words.toml")).unwrap()
});

/// All good words (words of length 4)
static GOOD_WORDS: LazyLock<Words> = LazyLock::new(|| {
    toml::from_str(include_str!("../assets/good_words.toml")).unwrap()
});

/// All best words (words of length 5)
static BEST_WORDS: LazyLock<Words> = LazyLock::new(|| {
    toml::from_str(include_str!("../assets/best_words.toml")).unwrap()
});

/// This enum is a smart pointer to all of the words in the catagory of its
/// variant. If `Self::Easy`, it's a `Words` that contains all 3-letter words,
/// if `Self::Good` all 4-letter words, and if `Self::Best` all 5-letter words.
/// It does not implement DerefMut, as each catagory is read at compile time.
#[derive(Clone, Copy, Debug, Default, GodotConvert, Var, Export)]
#[repr(u8)]
#[godot(via = u8)]
pub enum WordCategory {
    #[default]
    None,
    Easy,
    Good,
    Best,
}
impl Deref for WordCategory {
    type Target = Words;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::None => unimplemented!("WordCategory::None has no Words"),
            Self::Easy => &EASY_WORDS,
            Self::Good => &GOOD_WORDS,
            Self::Best => &BEST_WORDS,
        }
    }
}

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
    #[export]
    word: GString,
    #[var(get)]
    #[export]
    description: GString,
    #[var(get)]
    #[export]
    picture: Gd<Texture2D>,
    #[var(get)]
    #[export]
    audio: Gd<AudioStream>,
}

/// A mapping of words to the metadata to be used by `ExtShowWord`.
#[derive(Deserialize)]
pub struct Words(HashMap<String, String>);
impl Words {
    /// Constructs a word's metadata from a StringName, if it exists, to be
    /// used in an `ExtShowWord` scene.
    pub fn get(&self, word: StringName) -> Option<Gd<ExtWordMeta>> {
        let description = self.0.get(&word.to_string())?;

        let word = word.into();
        let description = description.into();
        let picture =
            load(format!("res://assets/licensed/pixabay/words/{}.webp", word));
        let audio =
            load(format!("res://assets/licensed/luvvoice/words/{}.mp3", word));

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

#[cfg(test)]
impl Words {
    fn as_inner(&self) -> &HashMap<String, String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // WARNING: Change this if module moved
    const GODOT_ROOT: &str = "../godot";

    #[test]
    fn words_all_have_valid_paths() {
        for category in
            [WordCategory::Easy, WordCategory::Good, WordCategory::Best]
        {
            for word in category.as_inner().keys() {
                let image = format!(
                    "{GODOT_ROOT}/assets/licensed/pixabay/words/{word}.webp"
                );
                assert!(
                    std::fs::exists(&image).is_ok_and(|exists| exists),
                    "{image} does not exist"
                );

                let audio = format!(
                    "{GODOT_ROOT}/assets/licensed/luvvoice/words/{word}.mp3"
                );
                assert!(
                    std::fs::exists(&audio).is_ok_and(|exists| exists),
                    "{audio} does not exist"
                );

                println!("{word} has valid paths")
            }
        }
    }
}
