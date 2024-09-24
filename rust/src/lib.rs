mod letter_manager;
mod letters;

use godot::prelude::*;

struct KindergartenWordFinderExtensionLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for KindergartenWordFinderExtensionLibrary {}
