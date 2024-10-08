use crate::{managers::letter::ExtLetterManager, words::WORDS};

use godot::prelude::*;

/// This node's main task is checking if words created by the ExtLetterManager
/// are words within the game dictionary, and changing the scene if so. It
/// expects at least one child, ExtLetterManager.
#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub struct ExtWordManager {
    base: Base<Node2D>,

    #[var(get)]
    #[export]
    show_word: Gd<PackedScene>,
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

        self.show_word = load("res://scenes/show_word.tscn");
    }
}
#[godot_api]
impl ExtWordManager {
    #[func]
    fn on_word_created(&mut self, word: StringName) {
        if let Some(word_meta) = WORDS.get(word) {
            let mut show_word = self.show_word.instantiate().unwrap();
            show_word.add_child(word_meta.upcast());

            let children = self.base().get_children();
            for mut child in children.iter_shared() {
                self.base_mut().remove_child(child.clone());
                child.queue_free();
            }
            self.base_mut().replace_by(show_word.clone());
        }
    }
}
