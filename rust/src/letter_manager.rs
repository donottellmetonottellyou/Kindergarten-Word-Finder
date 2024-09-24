use crate::letters::ExtLetter;

use godot::prelude::*;

use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

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
