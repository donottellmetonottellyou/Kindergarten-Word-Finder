use crate::{
    managers::letter::ExtLetterManager,
    words::{ExtWordMeta, WORDS},
};

use godot::prelude::*;

/// This node's main task is checking if words created by the ExtLetterManager
/// are words within the game dictionary, sending a signal if true, and telling
/// ExtLetterManager to clear the board otherwise (WIP).
#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub struct ExtWordManager {
    base: Base<Node2D>,
}
#[godot_api]
impl INode2D for ExtWordManager {
    fn ready(&mut self) {
        let mut letter_manager: Gd<ExtLetterManager> = self
            .base()
            .find_child("ExtLetterManager".into())
            .unwrap()
            .cast();

        letter_manager.connect(
            "word_created".into(),
            self.to_gd().callable("on_word_created"),
        );
    }
}
#[godot_api]
impl ExtWordManager {
    #[signal]
    fn valid_word_created(word: Gd<ExtWordMeta>);

    #[func]
    fn on_word_created(&mut self, word: StringName) {
        match WORDS.get(word) {
            Some(word_meta) => {
                godot_print!("It was a valid word: {word_meta}");
                self.to_gd().emit_signal(
                    "valid_word_created".into(),
                    &[word_meta.to_variant()],
                );
            }
            None => todo!(),
        };
    }
}
