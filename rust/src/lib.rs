pub mod letter_manager;
pub mod letters;
pub mod words;

use godot::prelude::*;

struct KindergartenWordFinderExtensionLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for KindergartenWordFinderExtensionLibrary {}
