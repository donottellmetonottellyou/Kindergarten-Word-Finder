mod letter_manager;
mod letters;
mod words;

use godot::prelude::*;

struct KindergartenWordFinderExtensionLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for KindergartenWordFinderExtensionLibrary {}
