use crate::letters::{ExtLetter, Letter};

use godot::prelude::*;

use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

/// A manager to manage all your letters! This class is in charge of moving and
/// notifying of changes in the letters in its scene.
///
/// On initialization, it expects two children, `TraySlots` and `WordSlots`.
/// Each of these should have any (reasonable) number of `Node2D`'s, numbered
/// from 0 on. `TraySlots` should additionally have one grandchild `ExtLetter`
/// per child.
#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub struct ExtLetterManager {
    base: Base<Node2D>,

    word_slots: FixedVec<Gd<Node2D>>,
    word_letters: FixedVec<Option<Gd<ExtLetter>>>,
    tray_slots: FixedVec<Gd<Node2D>>,
    tray_letters: FixedVec<Option<Gd<ExtLetter>>>,
}
#[godot_api]
impl INode2D for ExtLetterManager {
    fn ready(&mut self) {
        self.word_slots = (0_u8..)
            .map_while(|i| self.base().try_get_node_as(format!("WordSlots/{i}")))
            .collect::<Vec<_>>()
            .into();
        let word_slots_len = self.word_slots.len();
        self.word_letters = vec![None; word_slots_len].into();

        self.tray_slots = (0_u8..)
            .map_while(|i| self.base().try_get_node_as(format!("TraySlots/{i}")))
            .collect::<Vec<_>>()
            .into();
        let tray_slots_len = self.tray_slots.len();
        self.tray_letters = self
            .tray_slots
            .iter()
            .map(|node| node.try_get_node_as("ExtLetter"))
            .inspect(|letter| match letter {
                Some(_) => (),
                None => godot_error!("An available tray slot is missing a letter!"),
            })
            .collect::<Vec<_>>()
            .into();

        self.tray_letters
            .iter()
            .enumerate()
            .filter_map(|(i, letter)| letter.clone().map(|letter| (i, letter)))
            .for_each(|(i, mut letter)| {
                let letter_variant = letter.clone().to_variant();
                letter.bind_mut().connect_button_pressed(
                    self.to_gd()
                        .callable("on_letter_pressed")
                        .bindv(Array::from_iter([letter_variant, (i as u8).to_variant()])),
                );
            });

        if word_slots_len == 0 || tray_slots_len == 0 {
            godot_error!("Either word or letters has a length of zero");
        }
        if word_slots_len > tray_slots_len {
            godot_error!(
                "Word length {} is greater than {} available letters",
                word_slots_len,
                tray_slots_len
            );
        }
        godot_print!(
            "Initialized ExtLetterManager with {} letters for a {} length word",
            tray_slots_len,
            word_slots_len
        );
    }
}
#[godot_api]
impl ExtLetterManager {
    #[signal]
    fn word_created(word: StringName);

    /// This runs whenever any letter is clicked, moving it if it's possible.
    /// It will move the letter to the tray if it's in the word, and to the
    /// word if there is space in the word and it's in the tray.
    #[func]
    fn on_letter_pressed(&mut self, mut letter: Gd<ExtLetter>, i: u8) {
        if self.tray_letters[i as usize].is_some() {
            let first_free_position =
                match self.word_letters.iter().position(|letter| letter.is_none()) {
                    Some(i) => i,
                    None => return,
                };

            self.tray_slots[i as usize].remove_child(letter.clone().upcast());
            self.word_slots[first_free_position].add_child(letter.clone().upcast());

            letter.bind_mut().set_jiggle(false);

            std::mem::swap(
                &mut self.tray_letters[i as usize],
                &mut self.word_letters[first_free_position],
            );

            return self.check_if_word_created();
        }

        let word_position = match self.word_letters.iter().rposition(|word_letter| {
            word_letter
                .as_ref()
                .map_or(false, |word_letter| *word_letter == letter)
        }) {
            Some(i) => i,
            None => return,
        };

        self.word_slots[word_position].remove_child(letter.clone().upcast());
        self.tray_slots[i as usize].add_child(letter.clone().upcast());

        letter.bind_mut().set_jiggle(true);

        std::mem::swap(
            &mut self.tray_letters[i as usize],
            &mut self.word_letters[word_position],
        );
    }

    #[inline]
    fn check_if_word_created(&self) {
        let word = self
            .word_letters
            .iter()
            .filter_map(|letter| letter.clone())
            .map(|letter| letter.bind().get_letter())
            .fold(String::new(), |mut string, byte| {
                string.push((byte + b'a') as char);
                string
            });

        if word.len() < self.word_letters.len() {
            return;
        }

        godot_print!("Potential word {word} created");
        self.to_gd().emit_signal(
            "word_created".into(),
            &[StringName::from(word).to_variant()],
        );
    }
}

/// A fixed-size vector, which will only allow in-place modification after
/// initialization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct FixedVec<T>(Vec<T>);
impl<T> FixedVec<T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}
impl<T> AsRef<[T]> for FixedVec<T> {
    fn as_ref(&self) -> &[T] {
        self
    }
}
impl<T> AsMut<[T]> for FixedVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}
impl<T> Borrow<[T]> for FixedVec<T> {
    fn borrow(&self) -> &[T] {
        &self[..]
    }
}
impl<T> BorrowMut<[T]> for FixedVec<T> {
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}
impl<T> Default for FixedVec<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}
impl<T> Deref for FixedVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for FixedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> From<Vec<T>> for FixedVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}
impl<T> From<FixedVec<T>> for Vec<T> {
    fn from(value: FixedVec<T>) -> Self {
        value.0
    }
}
