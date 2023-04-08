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




## Vim commands & Vim Motions

Some basic commands to get started.

### Command mode

#### write (or save)
The `:w` (short for `:write`) allows you to write the current file, i.e. save it.

#### quit
Despite the numerous memes, all we need to ever quit Vim is `:q`


### Normal mode

#### Basic movement

Just like we can use the arrow keys to move around normally, in normal mode these can be replaced by the following:

Note: We can easily use the arrow keys to move around as well.

- `h` for **LEFT**
- `j` for **DOWN**
- `k` for **UP**
- `l` for **RIGHT** 

On MacOS keyboards, we can skip across words using the <kbd>⌥ Option</kbd> key with a left/right arrow. Similarly, Vim Motions has the same action but using two keys:

- `w` for moving forwards
- `b` for moving backwards

In Vim, a motion is anything which moves the cursor, and we have the "Command Count Motion" style, we can combine the command with a count to multiply it: for example if we want to move 5 lines down  we can write `5` followed by `j` and it will move down 5 lines. We can combine this with any command in Normal mode, essentially multiplying how much the command will acomplish. 

Below are some more commands:

- `d` for Deleting, `dd` will delete a line 
    We can use the Command Count Motion here as well, for example if we want to delete the line we are on, as well as the 4 lines below it: `d4j` will do just that for us.
    Similarly we can delete any number of words ahead, example: 4 words ahead: `d4w`.
- `u` to Undo the last action
- `Ctrl + r` redo's the previous action

### Insert mode

Insert mode is the primary mode in which we will be typing, there are two ways we can enter Insert mode:

- `i` to enter Insert mode to the left of your cursor
- `a` to enter Insert mode to the right of your cursor

In order to leave Insert mode we can use some of the following commands:

- `Esc`
- `Ctrl + [`
- `Ctrl + c` 

The last one (`Ctrl + c`) causes some weird behaviour in Visual mode (will be discussed later on)

### Visual mode 

Visual mode, the last mode to be discussed, is kind of like highlighting and manipulating lines on the screen how often we would usually do with a mouse.
It has two primary ways we can use it with: 

- The lowercase `v` lets us go into Visual mode and use our previously learnt motions, and automatically highlights the character where the cursor is
- The capital `V` (`Shift + v`) goes into Visual line mode, and automatically highlights the entire line of where the cursor is.

Once we are in visual mode, here are some commands to use with which we can interact with the text:

- `y` yanks the highlighted area, another word for copy
- `p` pastes, however differently in the two Visual modes:
    - In Visual mode normally, it pastes the exact content we copied
    - In Visual Line mode we paste in a new line.
    - Also note, that yanking and deleting go to the same buffer, so for example if we use `dd` to delete a line, and then `p` - it will paste the deleted line.