use crate::letters::ExtLetter;

use anyhow::{anyhow, Context, Result};
use godot::prelude::*;
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Error)]
pub enum LetterManagerError {
    #[error("Not loaded due to not being in scene tree")]
    NotInitialized,
    #[error("{0} not found in correct location")]
    NotFound(&'static str),
    #[error("This is not a valid slot in the current tree")]
    NotPartOfTree,
    #[error("No letter here")]
    NoLetter,
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct ExtLetterManager {
    word: [Result<Gd<ExtLetter>>; 5],
    letters: [Result<Gd<ExtLetter>>; 10],
    word_slots: [Result<Gd<Node2D>>; 5],
    letters_slots: [Result<Gd<Node2D>>; 10],

    base: Base<Node2D>,
}
#[godot_api]
impl INode2D for ExtLetterManager {
    fn init(base: Base<Self::Base>) -> Self {
        use LetterManagerError::NotInitialized;

        let word = std::array::from_fn(|_| Err(NotInitialized.into()));
        let letters = std::array::from_fn(|_| Err(NotInitialized.into()));
        let word_slots = std::array::from_fn(|_| Err(NotInitialized.into()));
        let letters_slots = std::array::from_fn(|_| Err(NotInitialized.into()));

        Self {
            word,
            letters,
            word_slots,
            letters_slots,

            base,
        }
    }

    fn ready(&mut self) {
        use LetterManagerError::{NoLetter, NotFound, NotPartOfTree};

        let word_boxes: Result<Gd<Node2D>, _> = self
            .base()
            .try_get_node_as("WordBoxes")
            .ok_or(NotFound("WordBoxes"));
        let letter_box: Result<Gd<Node2D>, _> = self
            .base()
            .try_get_node_as("LetterBox")
            .ok_or(NotFound("LetterBox"));

        let (found_word_slots, found_letters_slots) = match (word_boxes, letter_box) {
            (Ok(word_boxes), Ok(letter_box)) => {
                (word_boxes.get_children(), letter_box.get_children())
            }
            (Err(e), Ok(_)) | (Ok(_), Err(e)) => {
                for slot in self
                    .word_slots
                    .iter_mut()
                    .chain(self.letters_slots.iter_mut())
                {
                    *slot = Err(e).context("Parent was not found");
                }

                return godot_error!("{e}");
            }
            (Err(e1), Err(e2)) => {
                for slot in self
                    .word_slots
                    .iter_mut()
                    .chain(self.letters_slots.iter_mut())
                {
                    *slot = Err(e1).with_context(|| e2).context("Parent was not found");
                }

                return godot_error!("{e1}\n{e2}");
            }
        };

        let mut found_word_slots = found_word_slots.iter_shared();
        for (word_slot, letter) in self.word_slots.iter_mut().zip(self.word.iter_mut()) {
            match found_word_slots.next() {
                Some(found_word_slot) => {
                    *word_slot = found_word_slot
                        .try_cast::<Node2D>()
                        .map_err(|e| anyhow!("{e}"))
                        .context("Child should be a Node2D")
                        .inspect_err(|e| godot_error!("{e}"));
                    *letter = match word_slot {
                        Ok(_) => Err(NoLetter.into()),
                        Err(e) => Err(anyhow!("{e}")).context("Parent was not found"),
                    }
                }
                None => *word_slot = Err(NotPartOfTree.into()),
            }
        }

        let mut found_letters_slots = found_letters_slots.iter_shared();
        for (letters_slot, letter) in self.letters_slots.iter_mut().zip(self.letters.iter_mut()) {
            match found_letters_slots.next() {
                Some(found_letters_slot) => {
                    *letters_slot = found_letters_slot
                        .try_cast::<Node2D>()
                        .map_err(|e| anyhow!("{e}"))
                        .context("Child should be a Node2D")
                        .inspect_err(|e| godot_error!("{e}"));
                    *letter = match letters_slot {
                        Ok(letters_slot) => {
                            letters_slot.try_get_node_as("ExtLetter").ok_or(anyhow!(
                            "LettersSlot nodes should have one ExtLetter child in the scene tree"
                        ))
                        }
                        Err(e) => Err(anyhow!("{e}").context("Parent was not found")),
                    };
                }
                None => {
                    *letters_slot = Err(NotPartOfTree.into());
                    *letter = Err(NotPartOfTree.into());
                }
            }
        }
    }
}
