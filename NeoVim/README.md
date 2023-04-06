# Vim basics

I will personally use this to learn Vim as a text editor, as well as keep notes of any commands throughout.

The content used for learning Vim will be mixed of the following:
- [Learn Vim the Hard Way](https://learnvimscriptthehardway.stevelosh.com/)
- [ThePrimeagen on YouTube](https://www.youtube.com/watch?v=X6AR2RMB5tE&list=PLm323Lc7iSW_wuxqmKx_xxNtJC_hJbQ7R&ab_channel=ThePrimeagen)

## Vim Modes

Vim is a modal editor, below are some of the modes we should initially take a look at:

1. Normal mode - this is the mode you are in by default when inside a file, you can freely move around using arrow keys.
2. Insert mode - when entering insert mode a highlight will show up at the bottom saying **INSERT** to indicate you are in Insert mode.
This allows the user to insert information/data inside the file, AKA type around in the editor.
3. Visual mode - in visual mode we can highlight and look around text in the file, kind of like moving your mouse around.
4. Command mode - allows you to type commands, appears upon pressing `:`




## Vim commands

Some basic commands to get started.

### Command mode

#### write (or save)
The `:w` (short for `:write`) allows you to write the current file, i.e. save it.

#### quit
Despite the numerous memes, all we need to ever quit Vim is `:q`


### Normal mode

#### Basic movement

Just like we can use the arrow keys to move around normally, in normal mode these can be replaced by the following:

- `h` for **LEFT**
- `j` for **DOWN**
- `k` for **UP**
- `l` for **RIGHT** 

