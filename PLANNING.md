# Planning

Kindergarten Word Finder will be a 4+ educational game. It will feature various
difficulties of word creation and discovery.

## Gameplay

The player will start a game by selecting a difficulty. They will be tasked
with creating a word of a word length appropriate for the difficulty, with a
selection of letters that are twice the length of the word to be created. They
will be able to enter letters through clicking a letter or by entering it on
the keyboard. When they are successful, they will get feedback through a
displayed picture and a reward sound.

## Scenes

### Main Menu

- Should have 3 difficulty levels to choose from.
  - Easy will have 3 letter words
  - Normal will have 4 letter words
  - Hard will have 5 letter words
- Should be colorful and have an animated background, as well as a soundtrack.

### Finder View

- Should have 6-10 letter blocks to click on, depending on the difficulty.
- Should have 3-5 slots for the letter blocks to go in to.
- Should clear the slots if an unknown word is entered, and play an engaging
failure sound.
- Should be colorful and have an animated background, as well as a soundtrack
depending on the difficulty.

### Success View

- Should show the word that was found.
- Should play a short chime for success.
- Should say the word.
- Should show a picture related to the word.
- Should use the word in a sentence.
- Should be silent after this, to encourage replay.
- Should have a "Play Again" button.

## Music

For now, will be freely licensed music. This will be stored in the Godot folder
under Assets/Music/ArtistName. I will probably only use something licensed
under creative commons, such as Kevin MacLeod's music. I will keep a copy of the
license in the artist's folder, and attribute them in the main menu.

## Art

I will use pixel art drawn using Krita, but I will not be using a strict pixel
coordinate system. This is so that I can put the letters in random orientations,
to make the game more engaging and less mechanical.
