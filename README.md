# code-rustic
A small project for a terminal based code editor written in Rust.<br>
Currently using tui-rs (https://github.com/fdehau/tui-rs) and crossterm (https://github.com/crossterm-rs/crossterm) for dealing with interface and the terminal.

## Warnings

The code base for this project is very poorly documented and unorganized, but the next commit should be a refactor of all the code base.

The current state of the project is a demo for text editing in a buffer and some asynchronous testing for future development of an integrated terminal emulator.

## TODO (by priority)
* Code base refactoring. [0]
* Saving to and loading from files. [0]
* Some text manipulation tools (eg. find/replace, goto, etc.). [1]
* Asynchronous terminal emulator. [2]
* Syntax highlighting. [3]
* Code completion. [3] 
<br>
*Priority legend:* <br>
 0 : should be done imediately. <br> 
 1 : is an essential feature. <br> 
 2 : is needed. <br> 
 3 : will be done eventually. <br>
 4 : might not be implemented.


